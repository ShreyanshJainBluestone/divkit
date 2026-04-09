import re

with open("api_generator/api_generator/generators/rust/generator.py", "r") as f:
    gen_code = f.read()

# I need to insert generate_default_functions calls inside the struct's `impl {name}` block.
# Let's find where `pub fn builder() ->` is generated.
replacement = """
        result += f'    pub fn builder() -> {builder_name} {{\n'
        result += f'        {builder_name}::default()\n'
        result += '    }\n'
        
        from .rust_entities import generate_default_functions
        for prop in entity.properties:
            default_fn = generate_default_functions(prop, name)
            if default_fn:
                result += "    " + default_fn.replace("\\n", "\\n    ") + "\\n"
"""

gen_code = re.sub(r"        result \+\= f'    pub fn builder\(\) \-\> \{builder_name\} \{\\n'\n        result \+\= f'        \{builder_name\}::default\(\)\\n'\n        result \+\= '    \}\\n'", replacement.strip('\n'), gen_code)

with open("api_generator/api_generator/generators/rust/generator.py", "w") as f:
    f.write(gen_code)
