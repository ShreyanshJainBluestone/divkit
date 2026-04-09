import os
from typing import cast, Text

from ..base import Generator
from ...schema.modeling.entities import Declarable, Entity, EntityEnumeration, StringEnumeration
from ... import utils
from . import utils as rust_utils

class RustGenerator(Generator):
    def _main_declaration(self, obj: Declarable) -> Text:
        return super()._main_declaration(obj)

    def filename(self, name: str) -> str:
        return f'{utils.lower_camel_case(name).lower()}.rs'

    def full_rust_name(self, obj) -> str:
        name = utils.capitalize_camel_case(obj.name)
        current_parent = getattr(obj, "parent", None)
        while current_parent is not None:
            parent_name = utils.capitalize_camel_case(current_parent.name)
            if not name.startswith(parent_name):
                name = f'{parent_name}{name}'
            current_parent = getattr(current_parent, "parent", None)
        return name

    def _entity_declaration(self, entity: Entity) -> Text:
        
        is_template = hasattr(entity, 'generation_mode') and entity.generation_mode.is_template
        
        result = Text()
        if hasattr(entity, 'description') and entity.description:
            result += f'/// {entity.description.replace(chr(10), " ")}\n'
        result += '#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]\n'
        
        name = self.full_rust_name(entity)
        if is_template:
            name += "Template"
            
        result += f'pub struct {name} {{\n'
        
        from .rust_entities import format_rust_property, get_rust_type, rust_field_name
        for prop in entity.properties:
            result += format_rust_property(prop, name, is_template) + '\n'

        result += '}\n\n'
        
        # Builder pattern
        builder_name = f"{name}Builder"
        result += f'#[derive(Debug, Clone, Default)]\n'
        result += f'pub struct {builder_name} {{\n'
        
        for prop in entity.properties:
            field_name = rust_field_name(prop.name)
            rust_type = get_rust_type(prop.property_type, is_template)
            if prop.supports_expressions and not getattr(prop.property_type, '__class__').__name__ == 'Array':
                rust_type = f"Expression<{rust_type}>"
            if is_template:
                rust_type = f"Template<{rust_type}>"
            # Everything is Option in the builder
            result += f'    pub {field_name}: Option<{rust_type}>,\n'
            
        result += '}\n\n'
        
        result += f'impl {name} {{\n'
        result += f'    pub fn builder() -> {builder_name} {{\n'
        result += f'        {builder_name}::default()\n'
        result += '    }\n'
        
        from .rust_entities import generate_default_functions
        for prop in entity.properties:
            default_fn = generate_default_functions(prop, name)
            if default_fn:
                result += "    " + default_fn.replace("\n", "\n    ") + "\n"
        
        result += '}\n\n'
        
        result += f'impl {builder_name} {{\n'
        for prop in entity.properties:
            field_name = rust_field_name(prop.name)
            rust_type = get_rust_type(prop.property_type, is_template)
            if prop.supports_expressions and not getattr(prop.property_type, '__class__').__name__ == 'Array':
                rust_type = f"Expression<{rust_type}>"
            if is_template:
                rust_type = f"Template<{rust_type}>"
                
            orig_field_name_plain = field_name
            result += f'    pub fn {orig_field_name_plain}(mut self, value: {rust_type}) -> Self {{\n'
            result += f'        self.{field_name} = Some(value);\n'
            result += '        self\n'
            result += '    }\n'
            
        result += f'    pub fn build(self) -> {name} {{\n'
        result += f'        {name} {{\n'
        for prop in entity.properties:
            field_name = rust_field_name(prop.name)
            if not is_template and not prop.optional:
                is_static_string = prop.name == "type" and getattr(prop.property_type, '__class__').__name__ == "StaticString"
                if is_static_string:
                    val = prop.default_value if prop.default_value else prop.property_type.value
                    result += f'            {field_name}: String::from("{val}"),\n'
                elif prop.default_value and not is_template:
                    fn_name = f"{name}::default_{prop.name}()"
                    result += f'            {field_name}: self.{field_name}.or_else(|| Some({fn_name})).expect("missing required field \'{field_name}\'"),\n'
                else:
                    result += f'            {field_name}: self.{field_name}.expect("missing required field \'{field_name}\'"),\n'
            else:
                if prop.default_value and not is_template:
                    fn_name = f"{name}::default_{prop.name}()"
                    result += f'            {field_name}: self.{field_name}.or_else(|| Some({fn_name})),\n'
                else:
                    result += f'            {field_name}: self.{field_name},\n'
        result += '        }\n'
        result += '    }\n'
        result += '}\n'

        if entity.inner_types:
            for inner_type in sorted(entity.inner_types, key=lambda x: x.name):
                result += '\n'
                result += super()._main_declaration(inner_type)

        return result

    def _entity_enumeration_declaration(self, entity_enumeration: EntityEnumeration) -> Text:
        is_template = hasattr(entity_enumeration, 'mode') and getattr(entity_enumeration.mode, 'is_template', False)
        result = Text()
        if hasattr(entity_enumeration, 'description') and entity_enumeration.description:
            result += f'/// {entity_enumeration.description.replace(chr(10), " ")}\n'
        result += '#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]\n'
        result += '#[serde(tag = "type")]\n'
        
        name = self.full_rust_name(entity_enumeration)
        if is_template:
            name += "Template"
            
        result += f'pub enum {name} {{\n'
        
        for name_case, entity in zip(entity_enumeration.entity_names, entity_enumeration.entities):
            variant_name = utils.capitalize_camel_case(name_case)
            
            tag_value = name_case
            if entity[1] and hasattr(entity[1], "properties"):
                for prop in entity[1].properties:
                    if prop.name == "type":
                        if prop.property_type.__class__.__name__ == "StaticString":
                            tag_value = prop.property_type.value
                        elif prop.default_value:
                            tag_value = prop.default_value
                        break

            if entity[1]:
                inner_type = self.full_rust_name(entity[1])
            else:
                inner_type = variant_name
                
            if is_template:
                inner_type += "Template"
            result += f'    #[serde(rename = "{tag_value}")]\n'
            result += f'    {variant_name}({inner_type}),\n'
        
        result += '}\n'
        return result

    def _string_enumeration_declaration(self, string_enumeration: StringEnumeration) -> Text:
        result = Text()
        if hasattr(string_enumeration, 'description') and string_enumeration.description:
            result += f'/// {string_enumeration.description.replace(chr(10), " ")}\n'
        result += '#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]\n'
        
        name = self.full_rust_name(string_enumeration)
            
        result += f'pub enum {name} {{\n'
        
        for case in string_enumeration.cases:
            variant_name = rust_utils.rust_type_name(utils.capitalize_camel_case(case[0]))
            original_value = case[1]
            result += f'    #[serde(rename = "{original_value}")]\n'
            result += f'    {variant_name},\n'
        
        result += '}\n'
        return result
