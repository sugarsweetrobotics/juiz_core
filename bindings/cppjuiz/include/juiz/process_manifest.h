#pragma once

#include <string>
#include <cstdint>
#include <vector>
#include <map>
#include <optional>

#include "juiz/value.h"

class TopicManifest {
public:
  TopicManifest(const std::string& name): name_(name) {}

public:
  std::string name_;
};

class ArgumentManifest {
public:
  ArgumentManifest(const std::string& type_name, const std::string& name, const std::string& description, const juiz::Value& default_value):
    type_name_(type_name), name_(name), description_(description), default_(default_value) {}

public:
  juiz::Value into_value() const {
    return {
        {"name", name_},
        {"type", type_name_},
        {"description", description_},
        {"default", default_}
    };
  }
public:
  std::string type_name_;
  std::string name_;
  std::string description_;
  juiz::Value default_;
};

class ProcessManifest {
public:

  ProcessManifest(const std::string& type_name): type_name_(type_name), language_("c++"), factory_("process_factory") {}
  
  ProcessManifest description(const std::string& description) {
    description_ = description;
    return *this;
  }

  ProcessManifest name(const std::string& name) {
    name_ = name;
    return *this;
  }

  ProcessManifest language(const std::string& lang) {
    language_ = lang;
    return *this;
  }

  ProcessManifest factory(const std::string& fact) {
    factory_ = fact;
    return *this;
  }
  ProcessManifest add_bool_arg(const std::string& name, const std::string& description, bool default_value) {
    arguments_.push_back(ArgumentManifest("bool", name, description, juiz::Value{default_value}));
    return *this;
  }

  ProcessManifest add_int_arg(const std::string& name, const std::string& description, int64_t default_value) {
    arguments_.push_back(ArgumentManifest("int", name, description, {default_value}));
    return *this;
  }

  ProcessManifest add_float_arg(const std::string& name, const std::string& description, double default_value) {
    arguments_.push_back(ArgumentManifest("float", name, description, {default_value}));
    return *this;
  }

  ProcessManifest add_string_arg(const std::string& name, const std::string& description, const std::string& default_value) {
    arguments_.push_back(ArgumentManifest("string", name, description, {default_value}));
    return *this;
  }

  ProcessManifest use_memo(const bool use_memo) {
    use_memo_ = use_memo;
    return *this;
  }

  ProcessManifest container_type(const std::string& ct) {
    container_type_ = ct;
    return *this;
  }

  ProcessManifest container_name(const std::string& cn) {
    container_name_ = cn;
    return *this;
  }

public:
  juiz::Value into_value() const {
    std::vector<juiz::Value> args;
    for(auto i = arguments_.begin();i != arguments_.end(); ++i) {
        args.push_back(i->into_value());
    }
    juiz::Value v{
        {"type_name", type_name_},
        {"language", language_},
        {"description", description_},
        {"factory", factory_},
        {"arguments", args},
        {"use_memo", use_memo_},
    };
    if (name_) {
        v["name"] = juiz::Value{name_.value()};
    }
    return v;
  }
public:
  std::optional<std::string> name_;
  std::string type_name_;
  std::string description_;
  std::vector<ArgumentManifest> arguments_;
  std::string language_;
  std::string factory_;
  bool use_memo_;
  std::string broker_type_name_;
  std::string broker_name_;
  std::vector<TopicManifest> publishes_;
  std::map<std::string, TopicManifest> subscribes_;
  std::optional<std::string> container_name_;
  std::optional<std::string> container_type_;
};



class ContainerManifest {
public:

  ContainerManifest(const std::string& type_name): type_name_(type_name), language_("c++") {}

  ContainerManifest description(const std::string& description) {
    description_ = description;
    return *this;
  }

  ContainerManifest name(const std::string& name) {
    name_ = name;
    return *this;
  }

  ContainerManifest language(const std::string& lang) {
    language_ = lang;
    return *this;
  }

  ContainerManifest factory(const std::string& fact) {
    factory_ = fact;
    return *this;
  }

  ContainerManifest add_process(ProcessManifest&& process) {
    processes_.emplace_back(process);
    return *this;
  }

public:
  std::string type_name_;
  std::string language_;
  std::string factory_;
  std::string description_;
  juiz::Value args_;
  std::vector<ProcessManifest> processes_;
  std::optional<std::string> parent_type_name_;
  std::optional<std::string> parent_name_;
  std::optional<std::string> name_;

  juiz::Value into_value() const {
    
    juiz::Value v{
        {"type_name", type_name_},
        {"language", language_},
        {"description", description_},
        {"factory", factory_},
        {"args", args_}
    };
    if (name_) {
        v["name"] = juiz::Value{name_.value()};
    }
    return v;
  }
};