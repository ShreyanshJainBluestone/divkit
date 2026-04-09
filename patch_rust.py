import re

# Update rust_entities.py
with open("api_generator/api_generator/generators/rust/rust_entities.py", "r") as f:
    entities_code = f.read()

# Instead of returning None for validator we can translate the string
validator_func = """
def get_validator_attribute(prop_type: PropertyType) -> Optional[str]:
    # Very basic validator generation if needed
    return None
"""

entities_code = re.sub(r'def get_validator_attribute.*?(?=\n\n|\nfrom|\ndef)', validator_func, entities_code, flags=re.DOTALL)

with open("api_generator/api_generator/generators/rust/rust_entities.py", "w") as f:
    f.write(entities_code)

# Update generator.py
with open("api_generator/api_generator/generators/rust/generator.py", "r") as f:
    gen_code = f.read()

# Add builder default support
builder_build_replacement = """
        result += f'    pub fn build(self) -> {name} {{\n'
        result += f'        {name} {{\n'
        for prop in entity.properties:
            field_name = rust_field_name(prop.name)
            if not is_template and not prop.optional:
                # If it's a required discriminator type, hardcode it if it has a default!
                if prop.name == "type" and prop.default_value:
                    result += f'            {field_name}: String::from("{prop.default_value}"),\n'
                else:
                    result += f'            {field_name}: self.{field_name}.expect("missing required field \'{field_name}\'"),\n'
            else:
                if prop.default_value and not is_template:
                    from .rust_entities import get_rust_type
                    rust_type = get_rust_type(prop.property_type, False)
                    # Use unwrap_or_else to apply the default!
                    fn_name = f"{name}::default_{field_name}()"
                    result += f'            {field_name}: self.{field_name}.or_else(|| Some({fn_name})),\n'
                else:
                    result += f'            {field_name}: self.{field_name},\n'
        result += '        }\n'
        result += '    }\n'
"""

gen_code = re.sub(r"        result \+\= f'    pub fn build\(self\) \-\> \{name\} \{\\n'.*?        result \+\= '    \}\\n'", builder_build_replacement.strip('\n'), gen_code, flags=re.DOTALL)

with open("api_generator/api_generator/generators/rust/generator.py", "w") as f:
    f.write(gen_code)
