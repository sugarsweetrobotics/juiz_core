#pragma once

#include <functional>
#include <optional>
#include "process_manifest.h"
#include "value.h"
#include "capsule_map.h"

template<typename T>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>()> f) {
    return [=](juiz::CapsuleMap _) -> std::optional<T> { return f(); };
}


template<typename T>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const bool arg)> f) {
    return [=](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_bool(arg_name);
        return f(v); 
    };
}

template<typename T, typename... R>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const bool arg, R... arg2)> f) {
    return [iter, f](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const bool v = cm.get_bool(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_process<T>(i, std::function(binded))(cm); 
    };
}

template<typename T, typename IV, std::enable_if_t<std::is_integral<IV>::value, bool> = true>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const IV arg)> f) {
    return [=](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_int(arg_name);
        return f(v); 
    };
}

template<typename T, typename IV, typename... R, std::enable_if_t<std::is_integral<IV>::value, bool> = true>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const IV arg, R... arg2)> f) {
    return [iter, f](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const IV v = cm.get_int(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_process<T>(i, std::function(binded))(cm); 
    };
}

template<typename T, typename FV, std::enable_if_t<std::is_floating_point<FV>::value, bool> = true>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const FV arg)> f) {
    return [=](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_float(arg_name);
        return f(v); 
    };
}

template<typename T, typename FV, typename... R, std::enable_if_t<std::is_floating_point<FV>::value, bool> = true>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const FV arg, R... arg2)> f) {
    return [iter, f](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const FV v = cm.get_float(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_process<T>(i, std::function(binded))(cm); 
    };
}

template<typename T>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const std::string& str)> f) {
    return [=](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_string(arg_name);
        return f(v); 
    };
}

template<typename T, typename... R>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const std::string& arg, R... arg2)> f) {
    return [iter, f](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const std::string v = cm.get_string(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_process<T>(i, std::function(binded))(cm); 
    };
}

template<typename T>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const std::vector<juiz::Value>& str)> f) {
    return [=](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_array(arg_name);
        return f(v); 
    };
}

template<typename T, typename... R>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const std::vector<juiz::Value>& arg, R... arg2)> f) {
    return [iter, f](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const std::vector<juiz::Value> v = cm.get_array(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_process<T>(i, std::function(binded))(cm); 
    };
}


template<typename T>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const juiz::Value& str)> f) {
    return [=](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_object(arg_name);
        return f(v); 
    };
}

template<typename T, typename... R>
std::function<std::optional<T>(juiz::CapsuleMap)> bind_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(const juiz::Value& arg, R... arg2)> f) {
    return [iter, f](juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const auto v = cm.get_object(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_process<T>(i, std::function(binded))(cm); 
    };
}

////////



template<typename T, typename U>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*)> f) {
    return [=](U* u, juiz::CapsuleMap _) -> std::optional<T> { return f(u); };
}



template<typename T, typename U>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const bool arg)> f) {
    return [=](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_bool(arg_name);
        return f(u, v); 
    };
}

template<typename T, typename U, typename... R>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const bool arg, R... arg2)> f) {
    return [iter, f](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const auto v = cm.get_bool(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_container_process<T>(i, std::function(binded))(u, cm); 
    };
}

template<typename T, typename U, typename IV, std::enable_if_t<std::is_integral<IV>::value, bool> = true>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const IV arg)> f) {
    return [=](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_int(arg_name);
        return f(u, v); 
    };
}

template<typename T, typename U, typename IV, typename... R, std::enable_if_t<std::is_integral<IV>::value, bool> = true>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const IV arg, R... arg2)> f) {
    return [iter, f](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const IV v = cm.get_int(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_container_process<T>(i, std::function(binded))(u, cm); 
    };
}


template<typename T, typename U, typename FV, std::enable_if_t<std::is_floating_point<FV>::value, bool> = true>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const FV arg)> f) {
    return [=](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_float(arg_name);
        return f(u, v); 
    };
}

template<typename T, typename U, typename FV, typename... R, std::enable_if_t<std::is_floating_point<FV>::value, bool> = true>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const FV arg, R... arg2)> f) {
    return [iter, f](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const FV v = cm.get_float(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_container_process<T>(i, std::function(binded))(u, cm); 
    };
}


template<typename T, typename U>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const std::string& arg)> f) {
    return [=](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_string(arg_name);
        return f(u, v); 
    };
}

template<typename T, typename U, typename... R>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const std::string& arg, R... arg2)> f) {
    return [iter, f](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const auto v = cm.get_string(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_container_process<T>(i, std::function(binded))(u, cm); 
    };
}



template<typename T, typename U>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const std::vector<juiz::Value>& arg)> f) {
    return [=](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        auto v = cm.get_array(arg_name);
        return f(u, v); 
    };
}

template<typename T, typename U, typename... R>
std::function<std::optional<T>(U*, juiz::CapsuleMap)> bind_container_process(std::vector<ArgumentManifest>::iterator iter, std::function<std::optional<T>(U*, const std::vector<juiz::Value>& arg, R... arg2)> f) {
    return [iter, f](U* u, juiz::CapsuleMap cm) -> std::optional<T> {
        auto arg_name = (*iter).name_;
        const auto v = cm.get_array(arg_name);
        auto i = iter;
        ++i;
        auto binded = [v, f](R... rem) {
            return f(v, rem...);
        };
        return bind_container_process<T>(i, std::function(binded))(u, cm); 
    };
}

