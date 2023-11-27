from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives.asymmetric.utils import decode_dss_signature
from cryptography.hazmat.backends import default_backend

if __name__ == "__main__":
    message = input("Message to be signed: ")
    
    digest = hashes.Hash(hashes.SHA256(), backend=default_backend())
    digest.update(message.encode())
    hashed_value = digest.finalize()

    print("Hashed value: ", hashed_value.hex())
    
    elliptic_curve_key = ec.generate_private_key(ec.SECP256K1(), default_backend())
    
    signature = elliptic_curve_key.sign(hashed_value, ec.ECDSA(hashes.SHA256()))
    
    r, s = decode_dss_signature(signature)
    print(f"Signature (r): {r:x}")
    print(f"Signature (s): {s:x}")

    try:
        elliptic_curve_key.public_key().verify(signature, hashed_value, ec.ECDSA(hashes.SHA256()))
        print("Signature is valid.")
    except:
        print("Signature is NOT valid.")
