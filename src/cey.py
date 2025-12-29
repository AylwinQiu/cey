def read_file(path:str):
    with open(path, 'r') as f:
        ans = f.read()
    return ans

def skip_blank(src, where) -> int:
    "Return the position not the blank"
    ans:int = where
    while src[ans] in [' ', '\n']:
        ans += 1
    return ans

def expect_function(src:str, where:int):
    cursor = skip_blank(src, where)
    # Expect '{'
    if src[cursor]!='{':
        return None, where # Unmatch the {, return None
    # Match n Expr
    cursor += 1
    ans = ["Function"]
    tmp, cursor = expect_expr(src, cursor)
    while tmp!=None:
        ans.append(tmp)
        # Skip the ;
        cursor += 1
    cursor += 1 # Skip the }
    return ans, cursor

def expect_expr(src:str, where:int):
    # TODO
    return 1,2 # TODO

def expect_token(src:str, where:int):
    # number
    # string
    # function 
    # TODO
    return

def exec_function(vm, code_tree, mode):
    # TODO
    return

