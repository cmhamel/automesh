# Encrypted

We create an encrypted `.npy` file for the sake of testing.  We wish to test the case where the `.npy` file exists, it can be found, but it cannot be opened.

```python
from cryptography.fernet import Fernet
import numpy as np

# Generate a key for encryption
key = Fernet.generate_key()
cipher_suite = Fernet(key)

# Create a valid .npy file
data = np.array([1, 2, 3, 4, 5])
np.save('valid.npy', data)

# Read the file content
with open('valid.npy', 'rb') as f:
    file_data = f.read()

# Encrypt the file content
encrypted_data = cipher_suite.encrypt(file_data)

# Write the encrypted data to a new file
with open('encrypted.npy', 'wb') as f:
    f.write(encrypted_data)

print(f"Encryption key: {key.decode()}")
```
