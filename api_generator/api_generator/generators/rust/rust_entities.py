from typing import cast, List, Optional
from ...schema.modeling.entities import (
    PropertyType, String, Int, Double, Bool, BoolInt, Color, Url,
    Dictionary, Array, Entity, EntityEnumeration, StringEnumeration,
    Property, RawObject, RawArray, StaticString, Object, Declarable
)
from .utils import rust_field_name, rust_type_name
from ... import utils

def full_rust_name(obj) -> str:
    target = getattr(obj, "object", None)
    if target is None:
        target = obj
    
    name = utils.capitalize_camel_case(getattr(target, "resolved_name", target.name))

    current_parent = getattr(target, "parent", None)
    while current_parent is not None:
        parent_name = utils.capitalize_camel_case(getattr(current_parent, "resolved_name", current_parent.name))
        name = f'{parent_name}{name}'
        current_parent = getattr(current_parent, "parent", None)
        
    return name

def get_rust_type(prop_type: PropertyType, is_template: bool = False) -> str:
    if is_template and not isinstance(prop_type, Array): # simple template wrapper
        pass
    
    if isinstance(prop_type, String):
        return "String"
    elif isinstance(prop_type, Int):
        return "i64"
    elif isinstance(prop_type, Double):
        return "f64"
    elif isinstance(prop_type, (Bool, BoolInt)):
        return "bool"
    elif isinstance(prop_type, StaticString):
        return "String"
    elif isinstance(prop_type, Color):
        return "String"
    elif isinstance(prop_type, Url):
        return "String"
    elif isinstance(prop_type, Dictionary):
        return "std::collections::HashMap<String, serde_json::Value>"
    elif isinstance(prop_type, RawObject):
        return "serde_json::Value"
    elif isinstance(prop_type, RawArray):
        return "Vec<serde_json::Value>"
    elif isinstance(prop_type, Array):
        inner_type = get_rust_type(prop_type.property_type, is_template)
        return f"Vec<{inner_type}>"
    elif hasattr(prop_type, "name") or hasattr(prop_type, "resolved_name"):
        name = full_rust_name(prop_type)
        if is_template: name += "Template"
        return rust_type_name(name)
    else:
        return "serde_json::Value"



def get_validator_attribute(prop_type: PropertyType) -> Optional[str]:
    # Very basic validator generation if needed
    return None



def format_rust_property(prop: Property, struct_name: str, is_template: bool = False) -> str:
    original_name = prop.name
    field_name = rust_field_name(original_name)
    rust_type = get_rust_type(prop.property_type, is_template)
    
    if prop.supports_expressions and getattr(prop.property_type, '__class__').__name__ != 'Array':
        rust_type = f"Expression<{rust_type}>"
        
    if is_template:
        rust_type = f"Template<{rust_type}>"
    
    if prop.optional and not is_template:
        rust_type = f"Option<{rust_type}>"
        
    lines = []
    
    if prop.description:
        doc = prop.description.replace('\n', ' ')
        lines.append(f'    /// {doc}')

    if original_name != field_name.replace("r#", ""):
        lines.append(f'    #[serde(rename = "{original_name}")]')
    
    if not is_template:
        if prop.optional:
            lines.append(f'    #[serde(skip_serializing_if = "Option::is_none")]')
        elif prop.name == "type" and getattr(prop.property_type, '__class__').__name__ == "StaticString":
            # Enum discriminators are skipped by internally tagged enums in serde,
            # so we must default the type field if it exists.
            lines.append(f'    #[serde(default = "{struct_name}::default_type")]')
    else:
        lines.append(f'    #[serde(skip_serializing_if = "Option::is_none")]')
        
    lines.append(f'    pub {field_name}: {rust_type},')
    return "\n".join(lines)

def generate_default_functions(prop: Property, struct_name: str) -> str:
    is_static_string = getattr(prop.property_type, '__class__').__name__ == "StaticString"
    if not prop.default_value and not is_static_string:
        return ""
    
    val = prop.default_value if prop.default_value else (prop.property_type.value if is_static_string else None)
    rust_type = get_rust_type(prop.property_type, False)

    
    if prop.supports_expressions and getattr(prop.property_type, '__class__').__name__ != 'Array':
        type_str = f"Expression<{rust_type}>"
    else:
        type_str = rust_type
        
    fn_name = f"default_{prop.name}()"
    
    if rust_type == "String":
        val_str = f'"{val}".to_string()'
    elif rust_type == "bool":
        val_str = "true" if val else "false"
    elif rust_type == "f64":
        if isinstance(val, (int, float)):
            val_str = f"{float(val)}_f64"
        else:
            val_str = f"{val}_f64"
    elif rust_type == "i64":
        val_str = f"{val}i64"
    else:
        import json
        if isinstance(val, dict):
            # This is a complex object default. Serde JSON fallback is best for Rust defaults.
            val_json = json.dumps(val)
            val_str = f'serde_json::from_str(r#"{val_json}"#).unwrap()'
        else:
            try:
                # if struct type default value is a complex python object (like DivAnimation(name="fade",...)), json.dumps will fail.
                # but if it succeeds, it's a string enum/dict.
                if isinstance(val, str):
                    if val.startswith('{') or val.startswith('['):
                        val_json = val
                        val_str = f'serde_json::from_str(r#"{val_json}"#).unwrap()'
                    else:
                        val_json = json.dumps(val)
                        val_str = f'serde_json::from_str(r#"{val_json}"#).unwrap()'
                else:
                    val_json = json.dumps(val)
                    val_str = f'serde_json::from_str(r#"{val_json}"#).unwrap()'
            except Exception:
                if hasattr(val, 'name') and not isinstance(val, str):
                    enum_val = utils.capitalize_camel_case(str(getattr(val, 'name', str(val))))
                    val_str = f'{rust_type}::{enum_val}'
                elif str(val).startswith('{'):
                    val_json = str(val).replace("'", '"').replace('"', '\\\"')
                    val_str = f'serde_json::from_str("{val_json}").unwrap()'
                else:
                    enum_val = utils.capitalize_camel_case(str(val))
                    val_str = f'{rust_type}::{enum_val}'
    if prop.supports_expressions and getattr(prop.property_type, '__class__').__name__ != 'Array':
        val_str = f"Expression::value({val_str})"
        
    return f'pub fn {fn_name} -> {type_str} {{ {val_str} }}'
