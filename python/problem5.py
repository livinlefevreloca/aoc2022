import re

if __name__ == '__main__':
    mtx = [['' for _ in range(9)] for _ in range(8)]
    r = re.compile(r'[a-zA-Z]')
    s = """[C]         [S] [H]                
[F] [B]     [C] [S]     [W]        
[B] [W]     [W] [M] [S] [B]        
[L] [H] [G] [L] [P] [F] [Q]        
[D] [P] [J] [F] [T] [G] [M] [T]    
[P] [G] [B] [N] [L] [W] [P] [W] [R]
[Z] [V] [W] [J] [J] [C] [T] [S] [C]
[S] [N] [F] [G] [W] [B] [H] [F] [N]"""

    for i, line in enumerate(s.split('\n')):
        matches = r.finditer(line)
        for match in matches:
            print(match.span(0)[1], match.group(0))
            mtx[i][int((match.span(0)[1] - 2) / 4)] = match.group(0)

    print(mtx)

