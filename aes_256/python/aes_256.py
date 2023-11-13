from Crypto.Cipher import AES
from Crypto.Random import get_random_bytes
from Crypto.Util.Padding import pad, unpad

def aes_encrypt(key, iv, plaintext):
    cipher = AES.new(key, AES.MODE_CBC, iv)
    ciphertext = cipher.encrypt(pad(plaintext, AES.block_size))
    return ciphertext

def aes_decrypt(key, iv, ciphertext):
    cipher = AES.new(key, AES.MODE_CBC, iv)
    plaintext = unpad(cipher.decrypt(ciphertext), AES.block_size)
    return plaintext

def to_hex(data):
    return data.hex().upper()

if __name__ == "__main__":
    input_message = input("Enter the text to encrypt: ")
    plaintext = input_message.encode('utf-8')
    key = get_random_bytes(32)  # 256 bits for AES-256 random key
    iv = get_random_bytes(AES.block_size)

    ciphertext = aes_encrypt(key, iv, plaintext)
    hex_ciphertext = to_hex(ciphertext)
    print("Ciphertext:", hex_ciphertext)

    decrypted_text = aes_decrypt(key, iv, ciphertext)
    output_text = decrypted_text.decode('utf-8')
    print("Decrypted text:", output_text)
