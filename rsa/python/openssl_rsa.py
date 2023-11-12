from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives.asymmetric import rsa
from cryptography.hazmat.primitives.asymmetric import padding
from cryptography.exceptions import InvalidSignature   

if __name__ == '__main__':
    rsa_key = rsa.generate_private_key(
        public_exponent=65537,
        key_size=2048,
        backend=default_backend()
    )

    public_key = rsa_key.public_key()
    
    message = input("Enter a message to encrypt and decrypt: ").encode()
    
    encrypted_message = public_key.encrypt(
        message,
        padding.PKCS1v15()
    )

    print("Encrypted message:", encrypted_message.hex().upper())
    
    try:
        decrypted_message = rsa_key.decrypt(
            encrypted_message,
            padding.PKCS1v15()
        )
        print("Decrypted message:", decrypted_message.decode())

    except InvalidSignature as e:
        print("Decryption failed:", e)
