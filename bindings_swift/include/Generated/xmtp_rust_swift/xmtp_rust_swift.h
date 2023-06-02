// File automatically generated by swift-bridge.
#include <stdint.h>
#include <stdbool.h>
typedef struct RustSubscription RustSubscription;
void __swift_bridge__$RustSubscription$_free(void* self);

void* __swift_bridge__$Vec_RustSubscription$new(void);
void __swift_bridge__$Vec_RustSubscription$drop(void* vec_ptr);
void __swift_bridge__$Vec_RustSubscription$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_RustSubscription$pop(void* vec_ptr);
void* __swift_bridge__$Vec_RustSubscription$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_RustSubscription$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_RustSubscription$len(void* vec_ptr);
void* __swift_bridge__$Vec_RustSubscription$as_ptr(void* vec_ptr);

typedef struct RustClient RustClient;
void __swift_bridge__$RustClient$_free(void* self);

void* __swift_bridge__$Vec_RustClient$new(void);
void __swift_bridge__$Vec_RustClient$drop(void* vec_ptr);
void __swift_bridge__$Vec_RustClient$push(void* vec_ptr, void* item_ptr);
void* __swift_bridge__$Vec_RustClient$pop(void* vec_ptr);
void* __swift_bridge__$Vec_RustClient$get(void* vec_ptr, uintptr_t index);
void* __swift_bridge__$Vec_RustClient$get_mut(void* vec_ptr, uintptr_t index);
uintptr_t __swift_bridge__$Vec_RustClient$len(void* vec_ptr);
void* __swift_bridge__$Vec_RustClient$as_ptr(void* vec_ptr);

struct __private__ResultPtrAndPtr __swift_bridge__$RustSubscription$get_envelopes_as_query_response(void* self);
void __swift_bridge__$RustSubscription$close(void* self);
void __swift_bridge__$create_client(void* callback_wrapper, void __swift_bridge__$create_client$async(void* callback_wrapper, struct __private__ResultPtrAndPtr ret), void* host, bool is_secure);
void __swift_bridge__$RustClient$batch_query(void* callback_wrapper, void __swift_bridge__$RustClient$batch_query$async(void* callback_wrapper, struct __private__ResultPtrAndPtr ret), void* self, void* req);
void __swift_bridge__$RustClient$query(void* callback_wrapper, void __swift_bridge__$RustClient$query$async(void* callback_wrapper, struct __private__ResultPtrAndPtr ret), void* self, void* req);
void __swift_bridge__$RustClient$publish(void* callback_wrapper, void __swift_bridge__$RustClient$publish$async(void* callback_wrapper, struct __private__ResultPtrAndPtr ret), void* self, void* token, void* req);
void __swift_bridge__$RustClient$subscribe(void* callback_wrapper, void __swift_bridge__$RustClient$subscribe$async(void* callback_wrapper, struct __private__ResultPtrAndPtr ret), void* self, void* req);
void* __swift_bridge__$sha256(void* data);
void* __swift_bridge__$keccak256(void* data);
struct __private__ResultPtrAndPtr __swift_bridge__$verify_k256_sha256(void* public_key_bytes, void* message, void* signature, uint8_t recovery_id);
struct __private__ResultPtrAndPtr __swift_bridge__$diffie_hellman_k256(void* private_key_bytes, void* public_key_bytes);
struct __private__ResultPtrAndPtr __swift_bridge__$public_key_from_private_key_k256(void* private_key_bytes);
struct __private__ResultPtrAndPtr __swift_bridge__$recover_public_key_k256_sha256(void* message, void* signature);
struct __private__ResultPtrAndPtr __swift_bridge__$recover_public_key_k256_keccak256(void* message, void* signature);


