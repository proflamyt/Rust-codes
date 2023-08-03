hex1 = bytes.fromhex("1c0111001f010100061a024b53535009181c")
hex2 = bytes.fromhex("686974207468652062756c6c277320657965")

out = ''

o = zip(hex1, hex2)
    
for i,j in o:
    out += str(hex(i^j)).strip('0x')
    

print(out)