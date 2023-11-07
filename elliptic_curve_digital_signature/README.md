We chose to implement the Elliptic Curve Digital Signature Algorithm for this section, or ECDSA for short. As you will come to know if you read our paper, you will see that the signature does not increase or decrease its size depending on the size of the input message, something we saw happening with AES_256 and RSA. This is because we use the HASH256 cryptographic hashing function and then we continue with processing the message. We can do the same for the other two algorithms, albeit we have to pass the message through the hashing function after the processing. We also added a check whether the signature is valid or not (spoiler alert: it will always be valid since it was created a few lines before the check), in case it is needed to any other implementation we might come across in the future. 

NOTE: In the C++ program, we used smart pointers instead of raw pointers so we had to create custom deleters for the functions as OpenSSL requires custom deallocation functions. In case raw pointers are going to be used instead, then the following cleanup functions need to be called before exiting main.
- BN_free(k);
- BN_free(x);
- BN_free(y);
- EC_POINT_free(point);
- EC_KEY_free(elliptic_curve_key);
- ECDSA_SIG_free(signature);
We recommend to use smart pointers and not raw pointers, as with raw pointers there is the possibility or memory leaks if not they are not managed correctly or in case of exceptions. The only downside of smart pointers is that we have to be careful not to free the memory more than once.
