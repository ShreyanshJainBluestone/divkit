import re

modeling_path = 'STATIC_SDUI_DOMAIN_MODELING.md'
out_path = 'static_sdui_class_diagram.md'

with open(modeling_path, 'r') as f:
    content = f.read()

structs = re.findall(r'pub struct (\w+)(?:<.*?>)?\s*\{([^}]*)\}', content)
enums = re.findall(r'pub enum (\w+)\s*\{([^}]*)\}', content)

mermaid_lines = ["classDiagram"]
relations = set()

all_types = set([s[0] for s in structs] + [e[0] for e in enums])

def strip_types(t_str):
    return [f for f in re.findall(r'([A-Z][a-zA-Z0-9_]*)', t_str) if f in all_types]

for s_name, s_body in structs:
    mermaid_lines.append(f"  class {s_name} {{")
    for line in s_body.split('\n'):
        # match `pub name: type,` optionally with comments
        match = re.match(r'\s*pub\s+([a-zA-Z0-9_]+)\s*:\s*(.*?)(?:,|//|$)', line)
        if match:
            f_name = match.group(1).strip()
            f_type = match.group(2).strip()
            
            # Record relations BEFORE mutating f_type for display
            for rel in strip_types(f_type):
                if rel != s_name:
                    relations.add((s_name, rel))
            
            # FORMATTING FOR MERMAID
            # Remove spaces
            f_type_mermaid = f_type.replace(' ', '')
            # Mermaid handles generics with ~ but multiple type parameters separated by comma inside ~ can
            # sometimes be buggy, and MUST be completely closed `~...~`. 
            # I will use matched ~ instead of < and >
            f_type_mermaid = f_type_mermaid.replace('<', '~').replace('>', '~')
            
            mermaid_lines.append(f"    +{f_type_mermaid} {f_name}")
    mermaid_lines.append("  }")

for e_name, e_body in enums:
    mermaid_lines.append(f"  class {e_name} {{")
    mermaid_lines.append("    <<enumeration>>")
    
    # Strip comments from body
    e_body_clean = re.sub(r'//.*', '', e_body).replace('\n', '')
    
    for rel in strip_types(e_body_clean):
        if rel != e_name:
            relations.add((e_name, rel))
            
    # Since enums are comma separated and can have `Variant(Inner)`, split safely
    # Regex find all words before optional parens
    for v_match in re.findall(r'([A-Z][a-zA-Z0-9_]*)(?:\([^)]*\))?', e_body_clean):
        mermaid_lines.append(f"    {v_match}")
        
    mermaid_lines.append("  }")
    
for a, b in sorted(relations):
    mermaid_lines.append(f"  {a} --> {b}")

with open(out_path, 'w') as f:
    f.write("# SDUI Domain Class Diagram\n\n```mermaid\n")
    f.write("\n".join(mermaid_lines))
    f.write("\n```\n")
