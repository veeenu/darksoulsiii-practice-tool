origs = """
     0xE8, 0xAC, 0xC6, 0xFB, 0xFF, 0x90, 0x4D, 0x8B, 0xC7, 0x49, 0x8B, 0xD4, 0x48, 0x8B, 0xC8, 0xE8, 0x9D, 0xC6, 0xFB, 0xFF 
     0xE8, 0x8C, 0x07, 0xFC, 0xFF, 0x90, 0x4D, 0x8B, 0xC7, 0x49, 0x8B, 0xD4, 0x48, 0x8B, 0xC8, 0xE8, 0x7D, 0x07, 0xFC, 0xFF 
     0xE8, 0x0C, 0xBA, 0xFB, 0xFF, 0x90, 0x4D, 0x8B, 0xC7, 0x49, 0x8B, 0xD4, 0x48, 0x8B, 0xC8, 0xE8, 0xFD, 0xB9, 0xFB, 0xFF 
     0xE8, 0x9C, 0xBD, 0xFB, 0xFF, 0x90, 0x4D, 0x8B, 0xC7, 0x49, 0x8B, 0xD4, 0x48, 0x8B, 0xC8, 0xE8, 0x8D, 0xBD, 0xFB, 0xFF 
     0xE8, 0xAC, 0xC6, 0xFB, 0xFF, 0x90, 0x4D, 0x8B, 0xC7, 0x49, 0x8B, 0xD4, 0x48, 0x8B, 0xC8, 0xE8, 0x9D, 0xC6, 0xFB, 0xFF 
     0xE8, 0xAC, 0xC6, 0xFB, 0xFF, 0x90, 0x4D, 0x8B, 0xC7, 0x49, 0x8B, 0xD4, 0x48, 0x8B, 0xC8, 0xE8, 0x9D, 0xC6, 0xFB, 0xFF 
     0xE8, 0x1C, 0xBA, 0xFB, 0xFF, 0x90, 0x4D, 0x8B, 0xC7, 0x49, 0x8B, 0xD4, 0x48, 0x8B, 0xC8, 0xE8, 0x0D, 0xBA, 0xFB, 0xFF 
     0xE8, 0x1C, 0xBA, 0xFB, 0xFF, 0x90, 0x4D, 0x8B, 0xC7, 0x49, 0x8B, 0xD4, 0x48, 0x8B, 0xC8, 0xE8, 0x0D, 0xBA, 0xFB, 0xFF 
     0xE8, 0x1C, 0xBA, 0xFB, 0xFF, 0x90, 0x4D, 0x8B, 0xC7, 0x49, 0x8B, 0xD4, 0x48, 0x8B, 0xC8, 0xE8, 0x0D, 0xBA, 0xFB, 0xFF 
"""

patches = """
     0x48, 0x31, 0xC0, 0x48, 0x89, 0x02, 0x49, 0x89, 0x04, 0x24, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90 
"""

def to_arrays(s):
    lines = s.replace(' ',  '').strip().splitlines()
    lines = [ l.split(',') for l in lines ]
    lines = [ [int(j, 16) for j in l] for l in lines ]

    return lines

if __name__ == '__main__':
    origs = to_arrays(origs)
    aob = origs[0]

    for i in range(20):
        for j in origs[1:]:
            if aob[i] != j[i]:
                aob[i] = '??'

    aob = [ f'{i:X}' if i != '??' else '??' for i in aob ]
    print(' '.join(aob))
