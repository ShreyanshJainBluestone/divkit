import re

def rust_type_name(name: str) -> str:
    if name == 'Self':
        return 'Self_'
    return name

def rust_field_name(name: str) -> str:
    # Convert name to snake_case for fields
    name = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    name = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', name)
    name = name.lower().replace("-", "_")
    
    # Handling rust keywords
    reserved = {'type', 'struct', 'enum', 'fn', 'match', 'impl', 'for', 'loop', 'while', 'if', 'else', 'return'}
    if name in reserved:
        return f"r#{name}"
    return name
