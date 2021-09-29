# vertigo

List the chain of calls that end up to the given function. C codebases only.

## Options

```
[cpey@nuc ~]$ ./vertigo --help
vertigo 0.1.0

USAGE:
    vertigo --iterations <iterations> [search-functions]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --iterations <iterations>    Number of iterations

ARGS:
    <search-functions>...    Function names to get the call chain for
```

## Example

```
[cpey@nuc openssl]$ ./vertigo EVP_CipherUpdate -i 2
++ Call chain for EVP_CipherUpdate
 + Function: cms_kek_cipher, calling: EVP_CipherUpdate, iteration: 1
        path: crypto/cms/cms_kari.c
 + Function: CMS_RecipientInfo_kari_decrypt, calling: cms_kek_cipher, iteration: 2
        path: crypto/cms/cms_kari.c
 + Function: cms_RecipientInfo_kari_encrypt, calling: cms_kek_cipher, iteration: 2
        path: crypto/cms/cms_kari.c
 + Function: enc_read, calling: EVP_CipherUpdate, iteration: 1
        path: crypto/evp/bio_enc.c
 + Function: enc_write, calling: EVP_CipherUpdate, iteration: 1
        path: crypto/evp/bio_enc.c
 + Function: enc_ctrl, calling: enc_write, iteration: 2
        path: crypto/evp/bio_enc.c
 + Function: PKCS12_pbe_crypt, calling: EVP_CipherUpdate, iteration: 1
        path: crypto/pkcs12/p12_decr.c
 + Function: PKCS12_item_decrypt_d2i, calling: PKCS12_pbe_crypt, iteration: 2
        path: crypto/pkcs12/p12_decr.c
 + Function: PKCS12_item_i2d_encrypt, calling: PKCS12_pbe_crypt, iteration: 2
        path: crypto/pkcs12/p12_decr.c
 + Function: try_decode_PKCS8Encrypted, calling: PKCS12_pbe_crypt, iteration: 2
        path: crypto/store/loader_file.c
 + Function: ctr_BCC_block, calling: EVP_CipherUpdate, iteration: 1
        path: crypto/rand/drbg_ctr.c
 + Function: ctr_BCC_blocks, calling: ctr_BCC_block, iteration: 2
        path: crypto/rand/drbg_ctr.c
 + Function: ctr_BCC_init, calling: ctr_BCC_block, iteration: 2
        path: crypto/rand/drbg_ctr.c
 + Function: ctr_df, calling: EVP_CipherUpdate, iteration: 1
        path: crypto/rand/drbg_ctr.c
 + Function: ctr_update, calling: ctr_df, iteration: 2
        path: crypto/rand/drbg_ctr.c
 + Function: ctr_update, calling: EVP_CipherUpdate, iteration: 1
        path: crypto/rand/drbg_ctr.c
 + Function: drbg_ctr_instantiate, calling: ctr_update, iteration: 2
        path: crypto/rand/drbg_ctr.c
 + Function: drbg_ctr_reseed, calling: ctr_update, iteration: 2
        path: crypto/rand/drbg_ctr.c
 + Function: drbg_ctr_generate, calling: ctr_update, iteration: 2
        path: crypto/rand/drbg_ctr.c
 + Function: drbg_ctr_generate, calling: EVP_CipherUpdate, iteration: 1
        path: crypto/rand/drbg_ctr.c
 + Function: cipher_init, calling: EVP_CipherUpdate, iteration: 1
        path: providers/fips/self_test_kats.c
 + Function: EVP_PKEY_encrypt_init, calling: cipher_init, iteration: 2
        path: crypto/evp/pmeth_fn.c
 + Function: EVP_PKEY_decrypt_init, calling: cipher_init, iteration: 2
        path: crypto/evp/pmeth_fn.c
 + Function: cipher_ctrl, calling: cipher_init, iteration: 2
        path: engines/e_devcrypto.c
 + Function: self_test_cipher, calling: cipher_init, iteration: 2
        path: providers/fips/self_test_kats.c
 + Function: KRB5KDF, calling: cipher_init, iteration: 2
        path: providers/implementations/kdfs/krb5kdf.c
 + Function: self_test_cipher, calling: EVP_CipherUpdate, iteration: 1
        path: providers/fips/self_test_kats.c
 + Function: self_test_ciphers, calling: self_test_cipher, iteration: 2
        path: providers/fips/self_test_kats.c
 + Function: tls13_enc, calling: EVP_CipherUpdate, iteration: 1
        path: ssl/record/ssl3_record_tls13.c
 + Function: do_ssl3_write, calling: tls13_enc, iteration: 2
        path: ssl/record/rec_layer_s3.c
 + Function: test_tls13_encryption, calling: tls13_enc, iteration: 2
        path: test/tls13encryptiontest.c
 + Function: test_afalg_aes_cbc, calling: EVP_CipherUpdate, iteration: 1
        path: test/afalgtest.c
 + Function: encrypt_decrypt, calling: EVP_CipherUpdate, iteration: 1
        path: test/evp_fetch_prov_test.c
 + Function: test_EVP_CIPHER_fetch, calling: encrypt_decrypt, iteration: 2
        path: test/evp_fetch_prov_test.c
 + Function: cipher_test_enc, calling: EVP_CipherUpdate, iteration: 1
        path: test/evp_test.c
 + Function: cipher_test_run, calling: cipher_test_enc, iteration: 2
        path: test/evp_test.c
```
