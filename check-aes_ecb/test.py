def detect_aes_ecb(ciphertext, block_size=16):
    blocks = [ciphertext[i:i+block_size] for i in range(0, len(ciphertext), block_size)]
    unique_blocks = set(blocks)
    if len(unique_blocks) < len(blocks):
        print(len(blocks) - len(unique_blocks))
        return True
    else:
        return False

# Example usage:
ciphertext = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a"

if detect_aes_ecb(ciphertext):
    print("The ciphertext is likely encrypted using AES-ECB mode.")
else:
    print("The ciphertext does not appear to be encrypted using AES-ECB mode.")
