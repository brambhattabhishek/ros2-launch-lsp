import ast
import json

class LaunchVisitor(ast.NodeVisitor):
    def __init__(self):
        self.nodes = []
        self.substitutions = []
        self.variables = {}
    
    def visit_Call(self, node):
        if isinstance(node.func, ast.Name):
            if node.func.id == 'Node':
                node_info = {
                    "type": "Node",
                    "args": {},
                    "lineno": node.lineno,
                    "col_offset": node.col_offset
                }
                for kw in node.keywords:
                    node_info["args"][kw.arg] = ast.unparse(kw.value)
                self.nodes.append(node_info)
            elif node.func.id == 'LaunchConfiguration':
                if node.args and isinstance(node.args[0], ast.Str):
                    self.substitutions.append({
                        "name": node.args[0].s,
                        "lineno": node.lineno,
                        "col_offset": node.col_offset
                    })
        self.generic_visit(node)
    
    def visit_Assign(self, node):
        if isinstance(node.targets[0], ast.Name):
            var_name = node.targets[0].id
            if isinstance(node.value, ast.Call):
                if node.value.func.id == 'LaunchConfiguration':
                    if node.value.args and isinstance(node.value.args[0], ast.Str):
                        self.variables[var_name] = node.value.args[0].s
        self.generic_visit(node)

def parse_code(code: str) -> str:
    tree = ast.parse(code)
    visitor = LaunchVisitor()
    visitor.visit(tree)
    return json.dumps({
        "nodes": visitor.nodes,
        "substitutions": visitor.substitutions,
        "variables": visitor.variables
    })

# Execute the parser and store result
result = parse_code(ast.unparse(parsed))