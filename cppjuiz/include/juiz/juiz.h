#ifndef JUIZ_HEADER_FILE_INCLUDED
#define JUIZ_HEADER_FILE_INCLUDED



#ifdef __cplusplus
#include <cstdint>
#include "value.h"

extern "C" {
#else

#include <stdint.h>

#endif


typedef struct capsule_ptr_struct {
} capsule_ptr;

typedef struct capsule_struct {
} capsule;

typedef struct capsule_map_struct {
} capsule_map;

typedef struct value_struct {
} value;


int capsule_map_get_capsule(capsule_map* cm, const char* name, capsule_ptr** ptr);

int capsule_ptr_get_int(capsule_ptr* cp, int64_t* val);
int capsule_ptr_set_int(capsule_ptr* cp, int64_t val);
int capsule_ptr_get_uint(capsule_ptr* cp, uint64_t* val);
int capsule_ptr_get_bool(capsule_ptr* cp, int64_t* val);
int capsule_ptr_get_float(capsule_ptr* cp, double* val);
int capsule_ptr_get_string(capsule_ptr* cp, char** val);
int capsule_ptr_lock_as_value(capsule_ptr* cp, void callback(value*));

int capsule_ptr_lock_as_value_with_arg(capsule_ptr* cp, int64_t callback(void*, value*), void*);

int value_is_int(value* v);
int value_get_int(value* v, int64_t* int_value);
int value_is_uint(value* v);
int value_get_uint(value* v, uint64_t* uint_value);
int value_is_float(value* v);
int value_get_float(value* v, double* float_value);
int value_is_string(value* v);
int value_get_string(value* v, char** char_value);
int value_is_object(value* v);
int value_object_foreach(value* v, void callback(const char* key, value* v));
value* value_object_get_value(value*, const char* key);
int value_is_bool(value* v);
int value_get_bool(value* v, int* bool_value);
int value_is_array(value* v);
int value_is_array_foreach(value* v);
int value_is_null(value* v);

value* value_object_set_bool(value* v, const char* key, int64_t d);
value* value_object_set_int(value* v, const char* key, int64_t d);
value* value_object_set_uint(value* v, const char* key, uint64_t d);
value* value_object_set_float(value* v, const char* key, double d);
value* value_object_set_string(value* v, const char* key, const char* d);
value* value_object_set_empty_object(value* v, const char* key);
value* value_object_set_empty_array(value* v, const char* key);

value* value_array_push_bool(value* v, int64_t d);
value* value_array_push_int(value* v, int64_t d);
value* value_array_push_uint(value* v, uint64_t d);
value* value_array_push_float(value* v, double d);
value* value_array_push_string(value* v, const char* d);
value* value_array_push_empty_array(value* v);
value* value_array_push_empty_object(value* v);

int capsule_ptr_set_empty_object(capsule_ptr* cp);
int capsule_ptr_set_empty_array(capsule_ptr* cp);

int capsule_is_value(capsule* cp);
int capsule_is_int(capsule* cp);
int capsule_get_int(capsule* cp, int64_t* val);
int capsule_set_int(capsule* cp, int64_t val);


void print_value(value* value) ;
void print_key_value(const char* key, value* value) {
    printf("\"%s\" : ", key);
    print_value(value);
    printf(",");
}


void print_value(value* value) {
    if (value_is_object(value)) {
        printf("{");
        value_object_foreach(value, print_key_value);
        printf("}");
    } else if (value_is_int(value)) {
        int64_t i = 0;
        if (value_get_int(value, &i) != 0) {
            printf("\"value get_int error\"");
        }
        printf("int(%lld)", i);
    } else if (value_is_float(value)) {
        double i = 0;
        if (value_get_float(value, &i) != 0) {
            printf("\"value get_float error\"");
        }
        printf("float(%lf)", i);
    } else if (value_is_string(value)) {
        char* i;
        if (value_get_string(value, &i) != 0) {
            printf("\"value get_string error\"");
        }
        printf("\"%s\"", i);
    } 
}



#ifdef __cplusplus
}
#endif


//#ifdef __cplusplus

#include <optional>

namespace juiz {

    class ValueNotFoundError : public std::exception {
    };
    class CapsuleMap {
    private:
        capsule_map* _pmap;

    public:
        CapsuleMap(capsule_map* pmap) : _pmap(pmap) {}
        ~CapsuleMap() {}

    public:


        int64_t get_int(const std::string& name) const {
            capsule_ptr* ptr = NULL;
            if (capsule_map_get_capsule(this->_pmap, name.c_str(), &ptr) != 0) {
                throw ValueNotFoundError();
            }
            int64_t iv;
            if (capsule_ptr_get_int(ptr, &iv) != 0) {
                throw ValueNotFoundError();
            }
            return iv;
        }

        uint64_t get_uint(const std::string& name) const {
            capsule_ptr* ptr = NULL;
            if (capsule_map_get_capsule(this->_pmap, name.c_str(), &ptr) != 0) {
                throw ValueNotFoundError();
            }
            uint64_t iv;
            if (capsule_ptr_get_uint(ptr, &iv) != 0) {
                throw ValueNotFoundError();
            }
            return iv;
        }

        bool get_bool(const std::string& name) const {
            capsule_ptr* ptr = NULL;
            if (capsule_map_get_capsule(this->_pmap, name.c_str(), &ptr) != 0) {
                throw ValueNotFoundError();
            }
            int64_t iv;
            if (capsule_ptr_get_bool(ptr, &iv) != 0) {
                throw ValueNotFoundError();
            }
            return iv;
        }

        double get_float(const std::string& name) const {
            capsule_ptr* ptr = NULL;
            if (capsule_map_get_capsule(this->_pmap, name.c_str(), &ptr) != 0) {
                throw ValueNotFoundError();
            }
            double iv;
            if (capsule_ptr_get_float(ptr, &iv) != 0) {
                throw ValueNotFoundError();
            }
            return iv;
        }

        std::string get_string(const std::string& name) const {
            capsule_ptr* ptr = NULL;
            if (capsule_map_get_capsule(this->_pmap, name.c_str(), &ptr) != 0) {
                throw ValueNotFoundError();
            }
            char* iv;
            if (capsule_ptr_get_string(ptr, &iv) != 0) {
                throw ValueNotFoundError();
            }
            return std::string(iv);
        }
    };

    int64_t __set_value_obj_value(const Value* src_v, value* val);
    int64_t __set_value_array_value(const Value* src_v, value* val);

    inline int64_t _set_value_obj_callback(void* p, value* val) {
        juiz::Value* src_v = (Value*)p;
        return __set_value_obj_value(src_v, val);
    }

    inline int64_t __set_value_obj_value(const Value* src_v, value* val) {
        src_v->const_object_for_each([val](const std::string& k, const juiz::Value& v) {
            if (v.isBoolValue()) { value_object_set_bool(val, k.c_str(), v.boolValue() ? 1:0 ); }
            else if (v.isIntValue()) { value_object_set_int(val, k.c_str(), v.intValue()); }
            else if (v.isDoubleValue()) { value_object_set_float(val, k.c_str(), v.doubleValue()); }
            else if (v.isStringValue()) { value_object_set_string(val, k.c_str(), v.stringValue().c_str()); }
            else if (v.isObjectValue()) {
                auto elem = value_object_set_empty_object(val, k.c_str());
                __set_value_obj_value(&v, elem);
            } else if (v.isListValue()) {
                auto elem = value_object_set_empty_array(val, k.c_str());
                __set_value_array_value(&v, elem);
            }
        });
        return 0;
    }

    inline int64_t _set_value_array_callback(void* p, value* val) {
        juiz::Value* src_v = (Value*)p;
        return __set_value_array_value(src_v, val);
    }

    inline int64_t __set_value_array_value(const Value* src_v, value* val) {
        src_v->const_list_for_each([val](const juiz::Value& v) {
            if (v.isBoolValue()) { value_array_push_bool(val, v.boolValue() ? 1:0 ); }
            else if (v.isIntValue()) { value_array_push_int(val, v.intValue()); }
            else if (v.isDoubleValue()) { value_array_push_float(val, v.doubleValue()); }
            else if (v.isStringValue()) { value_array_push_string(val, v.stringValue().c_str()); }
            else if (v.isObjectValue()) {
                auto elem = value_array_push_empty_object(val);
                __set_value_obj_value(&v, elem);
            } else if (v.isListValue()) {
                auto elem = value_array_push_empty_array(val);
                __set_value_array_value(&v, elem);
            }
        });
        return 0;
    }
    
    int64_t capsule_ptr_set_value(capsule_ptr* ptr, Value v) {
        if (v.isObjectValue()) {
            if (capsule_ptr_set_empty_object(ptr) != 0) { return -1; }
            capsule_ptr_lock_as_value_with_arg(ptr, _set_value_obj_callback, (void*)&v);
        } else if (v.isListValue()) {
            if (capsule_ptr_set_empty_array(ptr) != 0) { return -1; }
            capsule_ptr_lock_as_value_with_arg(ptr, _set_value_array_callback, (void*)&v);
        }
        return 0;
    }
}


//#endif

extern "C" {
    int64_t manifest_entry_point(capsule_ptr* ptr);
    int64_t process_entry_point(capsule_map* cm, capsule* cp);
    int64_t (*process_factory())(capsule_map*,capsule*);
}

#define DEFINE_PROCESS_ENTRY_POINT(func, deser, ser)\
\
int64_t process_entry_point(capsule_map* cm, capsule* cp) {\
    try {\
        auto args = deser(juiz::CapsuleMap(cm));\
        if (!args) {\
            return -1;\
        }\
        auto return_value = std::apply(func, args.value());\
        if (!return_value) {\
            return -2;\
        }\
        return ser(cp, return_value.value());\
    } catch (juiz::ValueNotFoundError &e) {\
        return -11;\
    }\
}\
\
int64_t (*process_factory())(capsule_map*,capsule*) {\
    return process_entry_point;\
}


#define DEFINE_MANIFEST_ENTRY_POINT(manif) \
int64_t manifest_entry_point(capsule_ptr* ptr) { \
    auto v = manif(); \
    return capsule_ptr_set_value(ptr, v); \
}



#define DEFINE_CONTAINER_PROCESS_ENTRY_POINT(container_type, func, deser, ser)\
\
int64_t container_process_entry_point(container_type* container, capsule_map* cm, capsule* cp) {\
    try {\
        auto args = deser(juiz::CapsuleMap(cm));\
        if (!args) {\
            return -1;\
        }\
        auto return_value = std::apply(func, args.value());\
        if (!return_value) {\
            return -2;\
        }\
        return ser(cp, return_value.value());\
    } catch (juiz::ValueNotFoundError &e) {\
        return -11;\
    }\
}\
\
int64_t (*container_process_factory())(container_type*,capsule_map*,capsule*) {\
    return container_process_entry_point;\
}

#endif