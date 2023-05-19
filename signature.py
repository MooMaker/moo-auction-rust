from web3 import Web3, HTTPProvider
from eth_account.messages import encode_defunct

w3 = Web3(Web3.HTTPProvider('https://mainnet.infura.io/v3/'))  # Add Infura key Initialize w3

def sign_message(account, message):
    message_encoded = encode_defunct(text=message)
    signed_message = account.sign_message(message_encoded)
    return signed_message.signature.hex

account = w3.eth.account.from_key('0x4c0883a69102937d6231471b5dbb6204fe5129617082792ae468d01a3f362318')  # Replace with your private key
message = "Hello, world!"
signature = sign_message(account, message)

print(signature)
