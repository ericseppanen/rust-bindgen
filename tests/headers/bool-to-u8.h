// bindgen-flags: --bool-to-u8

typedef unsigned char uint8_t;

struct contains_bool {
        uint8_t a_byte;
        _Bool a_bool;
};
