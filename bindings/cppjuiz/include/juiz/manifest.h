#pragma once
#include "core.h"
#include <cstdint>


extern "C" {
    int64_t process_function_entry_point(capsule_map* cm, capsule* cp);
    int64_t manifest_entry_point(capsule_ptr* ptr);
    int64_t component_manifest_entry_point(capsule_ptr* ptr);
    int64_t process_entry_point(capsule_map* cm, capsule* cp);
    int64_t process_entry_point(capsule_map* cm, capsule* cp);
    int64_t (*process_factory_entry_point())(capsule_map*,capsule*);

    int64_t (*container_factory_entry_point())(value*, void**);
    int64_t (*container_process_factory_entry_point())(void*, capsule_map*,capsule*);

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


#define DEFINE_COMPONENT_MANIFEST_ENTRY_POINT(manif) \
int64_t component_manifest_entry_point(capsule_ptr* ptr) { \
    auto v = manif(); \
    return capsule_ptr_set_value(ptr, v); \
}

int64_t serialize(capsule* cp, int64_t retval) {
    return capsule_set_int(cp, retval);
}

template<bool>
int64_t serialize(capsule* cp, bool retval) {
    return capsule_set_bool(cp, retval);
}

template<double&>
int64_t serialize(capsule* cp, double& retval) {
    return capsule_set_float(cp, retval);
}

// template<std::string&>
int64_t serialize(capsule* cp, const std::string& retval) {
    return capsule_set_string(cp, retval.c_str());
}

int64_t serialize(capsule* cp, const std::string&& retval) {
    return capsule_set_string(cp, retval.c_str());
}

#define PROCESS_FACTORY(manifest_function, process_function) \
int64_t manifest_entry_point(capsule_ptr* ptr) { \
    auto v = manifest_function(); \
    return capsule_ptr_set_value(ptr, v); \
}\
int64_t process_entry_point(capsule_map* cm, capsule* cp) {\
    try {\
        auto return_value = process_function(juiz::CapsuleMap(cm));\
        if (!return_value) {\
            return JUIZ_PROCESS_FUNCTION_NULL_OPT_RETURNED;\
        }\
        auto v = return_value.value();\
        return serialize(cp, v);\
    } catch (juiz::ValueNotFoundError &e) {\
        return JUIZ_VALUE_NOT_FOUND_ERROR;\
    } catch (juiz::ValueConvertError &e) {\
        return JUIZ_VALUE_CONVERTER_ERROR;\
    }\
}\
\
int64_t (*process_factory_entry_point())(capsule_map*,capsule*) {\
    return process_entry_point;\
}


#define CONTAINER_FACTORY(manifest_function, construct_function, destruct_function) \
int64_t manifest_entry_point(capsule_ptr* ptr) { \
    auto v = manifest_function(); \
    return capsule_ptr_set_value(ptr, v); \
}\
int64_t container_create_entry_point(value* v, void** container_obj) {\
    try {\
        *container_obj = construct_function(juiz::into_value(v));\
        return JUIZ_OK;\
    } catch (juiz::ValueNotFoundError &e) {\
        return JUIZ_VALUE_NOT_FOUND_ERROR;\
    } catch (juiz::ValueConvertError &e) {\
        return JUIZ_VALUE_CONVERTER_ERROR;\
    }\
}\
\
int64_t (*container_factory_entry_point())(value*, void**) {\
    return container_create_entry_point;\
}


#define CONTAINER_PROCESS_FACTORY(container_type_t, manifest_function, process_function) \
int64_t manifest_entry_point(capsule_ptr* ptr) { \
    auto v = manifest_function(); \
    return capsule_ptr_set_value(ptr, v); \
}\
int64_t container_process_entry_point(void* container, capsule_map* cm, capsule* cp) {\
    try {\
        auto return_value = process_function((container_type_t*)(container), juiz::CapsuleMap(cm));\
        if (!return_value) {\
            return JUIZ_CONTAINER_PROCESS_FUNCTION_NULL_OPT_RETURNED;\
        }\
        auto v = return_value.value();\
        return serialize(cp, v);\
    } catch (juiz::ValueNotFoundError &e) {\
        return JUIZ_VALUE_NOT_FOUND_ERROR;\
    } catch (juiz::ValueConvertError &e) {\
        return JUIZ_VALUE_CONVERTER_ERROR;\
    }\
}\
\
int64_t (*container_process_factory_entry_point())(void*, capsule_map*,capsule*) {\
    return container_process_entry_point;\
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
        return JUIZ_VALUE_NOT_FOUND_ERROR;\
    } catch (juiz::ValueConvertError &e) {\
        return JUIZ_VALUE_CONVERTER_ERROR;\
    }\
}\
\
int64_t (*container_process_factory())(container_type*,capsule_map*,capsule*) {\
    return container_process_entry_point;\
}
