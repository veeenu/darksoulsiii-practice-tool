import json
import pprint

pp = pprint.PrettyPrinter(indent=2)

def count_wsp(s):
    i = 0
    for c in s:
        if c == ' ':
            i += 1
        else:
            return i

def count_indent(s):
    return int(count_wsp(s) / 2)

tokens = []
indent = 0

with open('item_list.txt', 'r') as fp:
    L = 0
    for line in fp:
        line = line.rstrip()
        sline = line.strip()
        if sline == '':
            continue

        li = count_indent(line)

        if li > indent:
            for i in range(li - indent):
                tokens.append('INDENT')
            tokens.append(sline)
        elif li == indent:
            tokens.append(sline)
        else:
            for i in range(indent - li):
                tokens.append('DEDENT')
            tokens.append(sline)

        indent = li

root = { 'node': 'root', 'children': [] }
stack = [root]

for t in tokens:
    try:
        if t == 'INDENT':
            stack.append(stack[-1]['children'][-1])
        elif t == 'DEDENT':
            stack.pop()
        else:
            if ':' in t:
                (desc, x) = t.split(':')
                node = { 'id': x, 'desc': desc }
            else:
                node = { 'node': t, 'children': [] }
            stack[-1]['children'].append(node)
    except Exception as e:
        raise e

print(json.dumps(root, indent=2))
