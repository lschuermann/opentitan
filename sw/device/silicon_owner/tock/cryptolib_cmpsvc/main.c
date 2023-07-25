/* #include "main.h" */

#include "sw/device/lib/crypto/impl/integrity.h"
#include "sw/device/lib/crypto/impl/keyblob.h"
#include "sw/device/lib/crypto/include/datatypes.h"
#include "sw/device/lib/crypto/include/mac.h"

int const CONSTNUMARR[64] = {
 26840, 19275, 26064, 27828, 57676, 19859, 3947, 25588, 41700, 40697, 40329, 11102, 6289, 606, 2743, 48741, 1128, 45315, 34812, 19564, 12270, 57406, 51768, 5209, 51430, 27753, 46415, 55106, 36712, 43261, 3113, 53868, 10325, 51222, 28607, 38436, 30444, 30914, 21983, 62479, 4819, 34505, 46516, 27770, 58781, 46802, 19796, 42607, 49886, 26116, 25490, 15969, 6968, 5022, 6152, 2440, 65459, 42714, 21557, 36048, 61721, 2188, 43148, 19778
};

int testclib_add(int a, int b) {
  int c = a + b;
  for (int i = 0; i < 64; i++) {
    c += CONSTNUMARR[i];
  }
  return c;
}


// HMAC functest

enum {
  /**
   * HMAC-SHA256 tag length (256 bits) in words.
   */
  kTagLenWords = 256 / 32,
};

// 256-bit test key (big endian) =
// 0x1bff10eaa5b9b204d6f3232a573e8e51a27b68c319366deaf26b91b0712f7a34
static const uint32_t kBasicTestKey[] = {
    0xea10ff1b, 0x04b2b9a5, 0x2a23f3d6, 0x518e3e57,
    0xc3687ba2, 0xea6d3619, 0xb0916bf2, 0x347a2f71,
};

/* // Short test key, 32 bits (big endian) = 0x1bff10ea */
/* static const uint32_t kShortTestKey[] = { */
/*     0xea10ff1b, */
/* }; */

// Long test key, 544 bits (big endian) =
// 0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f40414243
/* static const uint32_t kLongTestKey[] = { */
/*     0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c, 0x13121110, 0x17161514, */
/*     0x1b1a1918, 0x1f1e1d1c, 0x23222120, 0x27262524, 0x2b2a2928, 0x2f2e2d2c, */
/*     0x33323130, 0x37363534, 0x3b3a3938, 0x3f3e3d3c, 0x43424140, */
/* }; */

// Random value for masking, as large as the longest test key. This value
// should not affect the result.
static const uint32_t kTestMask[68] = {
    0x8cb847c3, 0xc6d34f36, 0x72edbf7b, 0x9bc0317f, 0x8f003c7f, 0x1d7ba049,
    0xfd463b63, 0xbb720c44, 0x784c215e, 0xeb101d65, 0x35beb911, 0xab481345,
    0xa7ebc3e3, 0x04b2a1b9, 0x764a9630, 0x78b8f9c5, 0x3f2a1d8e,
};

const char plaintext[] = "Test message.";

static int run_hmac_functest(uint8_t* data) {
  crypto_const_uint8_buf_t msg_buf = {
      .data = (uint8_t*)plaintext,
      .len = 1,
  };

  // Construct blinded key.
  crypto_key_config_t config = {
      .version = kCryptoLibVersion1,
      .key_mode = kKeyModeHmacSha256,
      .key_length = 32,
      .hw_backed = kHardenedBoolFalse,
      .diversification_hw_backed = {.data = NULL, .len = 0},
      .exportable = kHardenedBoolFalse,
      .security_level = kSecurityLevelLow,
  };

  uint32_t keyblob[keyblob_num_words(config)];
  status_t res = keyblob_from_key_and_mask(kBasicTestKey, kTestMask, config, keyblob);
  crypto_blinded_key_t blinded_key = {
      .config = config,
      .keyblob = keyblob,
      .keyblob_length = sizeof(keyblob),
      .checksum = 0,
  };
  blinded_key.checksum = integrity_blinded_checksum(&blinded_key);

  uint32_t act_tag[kTagLenWords];
  crypto_uint8_buf_t tag_buf = {
      .data = (unsigned char *)act_tag,
      .len = sizeof(act_tag),
  };

  res = otcrypto_hmac(&blinded_key, msg_buf, &tag_buf);
  memcpy(&data[0], (uint8_t*) &act_tag[0], 32);
  return res.value;
  /* TRY_CHECK_ARRAYS_EQ(act_tag, exp_tag, kTagLenWords); */
  /* return OK_STATUS(); */
}

/* static status_t cryptolib_cmpsvc_hmac(uint32_t* key, size_t key_length, uint8_t* data, size_t data_length) { */
/*   crypto_const_uint8_buf_t msg_buf = { */
/*       .data = data, */
/*       .len = data_length, */
/*   }; */

/*   // Construct blinded key. */
/*   crypto_key_config_t config = { */
/*       .version = kCryptoLibVersion1, */
/*       .key_mode = kKeyModeHmacSha256, */
/*       .key_length = 32, */
/*       .hw_backed = kHardenedBoolFalse, */
/*       .diversification_hw_backed = {.data = NULL, .len = 0}, */
/*       .exportable = kHardenedBoolFalse, */
/*       .security_level = kSecurityLevelLow, */
/*   }; */

/*   uint32_t keyblob[keyblob_num_words(config)]; */
/*   TRY(keyblob_from_key_and_mask(kBasicTestKey, kTestMask, config, keyblob)); */
/*   crypto_blinded_key_t blinded_key = { */
/*       .config = config, */
/*       .keyblob = keyblob, */
/*       .keyblob_length = sizeof(keyblob), */
/*       .checksum = 0, */
/*   }; */
/*   blinded_key.checksum = integrity_blinded_checksum(&blinded_key); */

/*   uint32_t act_tag[kTagLenWords]; */
/*   crypto_uint8_buf_t tag_buf = { */
/*       .data = (unsigned char *)act_tag, */
/*       .len = sizeof(act_tag), */
/*   }; */

/*   res = otcrypto_hmac(&blinded_key, msg_buf, &tag_buf); */
/*   memcpy(&data[0], (uint8_t*) &act_tag[0], 32); */

/*   return OK_STATUS(); */
/*   /\* TRY_CHECK_ARRAYS_EQ(act_tag, exp_tag, kTagLenWords); *\/ */
/*   /\* return OK_STATUS(); *\/ */
/* } */

static size_t keyblob_num_words_wrapped(const crypto_key_config_t* config) {
  return keyblob_num_words(*config);
}

static status_t keyblob_from_key_and_mask_wrapped(const uint32_t *key, const uint32_t *mask,
			  const crypto_key_config_t* config,
			  uint32_t *keyblob) {
  return keyblob_from_key_and_mask(key, mask, *config, keyblob);
}

static absl_status_t otcrypto_hmac_wrapped(const crypto_blinded_key_t *key,
		             crypto_const_uint8_buf_t* input_message,
		             crypto_uint8_buf_t *tag) {
  crypto_status_t status = otcrypto_hmac(key, *input_message, tag);
  return status_err(status);
}


typedef void (*fnptr)(void);

fnptr const
__attribute__ ((section (".contsvc_hdr")))
contsvc_fntab[18] = {
  (fnptr) testclib_add,

  // keyblob.h
  (fnptr) keyblob_num_words_wrapped,            // 1
  (fnptr) keyblob_share_num_words,              // 2
  (fnptr) keyblob_to_shares,                    // 3
  (fnptr) keyblob_from_shares,                  // 4
  (fnptr) keyblob_from_key_and_mask_wrapped,    // 5
  (fnptr) keyblob_remask,                       // 6

  // mac.h
  (fnptr) otcrypto_mac_keygen,                  // 7
  (fnptr) otcrypto_hmac_wrapped,                // 8
  (fnptr) otcrypto_kmac,                        // 9
  (fnptr) otcrypto_hmac_init,                   // 10
  (fnptr) otcrypto_hmac_update,                 // 11
  (fnptr) otcrypto_hmac_final,                  // 12

  // integrity.h
  (fnptr) integrity_unblinded_checksum,         // 13
  (fnptr) integrity_blinded_checksum,           // 14
  (fnptr) integrity_unblinded_key_check,        // 15
  (fnptr) integrity_blinded_key_check,          // 16

  (fnptr) run_hmac_functest,                    // 17
};
