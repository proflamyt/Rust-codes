# AES ECB

Find which of the texts are encrypted with ECB

Steps taken during AES-ECB Encryption 

- The plaintext is divided in blocks of 16 bytes
- Each block of plain text is encrypted to give a same sized ciphertext
- The ciphertexts are merged together.




#### Methods to detect potential ECB encryption

- Block Repeats: In ECB mode, identical plaintext blocks result in identical ciphertext blocks. So, one way to detect ECB encryption is to look for repeated blocks in the ciphertext. If you find two or more identical ciphertext blocks, it suggests that ECB may have been used. Keep in mind that this method may produce false positives, as identical blocks can occur in other encryption modes or due to coincidental patterns in the plaintext.

- Character Frequency Analysis: In some cases, you can perform character frequency analysis on the ciphertext. If you notice that the same characters or sequences of characters in the plaintext consistently map to the same ciphertext blocks, it may indicate the use of ECB. However, this method is not foolproof and may not work for all types of data.

- Statistical Analysis: You can perform statistical analysis on the ciphertext to look for patterns or anomalies. ECB tends to preserve statistical properties of the plaintext within each block, which can lead to detectable patterns in the ciphertext. This method is more complex and may require specialized tools or algorithms.

- Known Plaintext Attack: If you have access to the plaintext and its corresponding ciphertext, you can compare the ciphertext blocks with each other. If you observe that the same plaintext blocks always produce the same ciphertext blocks, it's a strong indicator of ECB. However, this method requires knowledge of the original plaintext.


Method used here is the repeating block method, here the ciphertexts is splitted into bytes of 16 . each then compared to check if it has occured before