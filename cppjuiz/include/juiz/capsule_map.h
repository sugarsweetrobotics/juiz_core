#pragma once

#include "core.h"

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
