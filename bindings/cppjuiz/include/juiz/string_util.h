#pragma once

#include <vector>
#include <string>

namespace juiz {

    /**
     * 文字列内の特定文字列の置き換え
     * @param ss 置き換え元
     * @param p パターン
     * @param m 置き換え文字列
     * @return 置き換え後の文字列
     */
    inline std::string replaceAll(const std::string ss, const std::string& p, const std::string& m) {
        auto s = ss; // コピーする
        auto pos = s.find(p);
        int toLen = m.length();
    
        // もしパターンが空の文字列ならば何もしない
        if (p.empty()) { return s;}
    
        while ((pos = s.find(p, pos)) != std::string::npos) {
            s.replace(pos, p.length(), m);
            pos += toLen;
        }
        return s;
    }

    inline std::string stringJoin(const std::vector<std::string>& strs, const char sep) {
        std::string buf;
        for(int i = 0;i < strs.size();i++) {
            buf += strs[i];
            if (i != strs.size()-1) buf += sep;
        }
        return buf;
    }

    inline std::string stringJoin(const std::vector<std::string>& strs) {
        std::string buf;
        for(int i = 0;i < strs.size();i++) {
            buf += strs[i];
        }
        return buf;
    }

    inline std::string stringJoin(std::vector<std::string>::iterator starts, const std::vector<std::string>::const_iterator& ends, const char sep) {
        std::string buf;
        while(true) {
            buf += *starts;
            ++starts;
            if (starts == ends) break;
            buf += sep;
        }
        return buf;
    }

    inline std::vector<std::string> stringSplit(const std::string& str, const char sep=' ') {
        std::vector<std::string> result;
        int i = 0;
        do {
            const int begin = i;
            ///const char *begin = pstr;
            while(str[i] != sep && str[i] != 0)
                i++;
            auto v = str.substr(begin, i-begin);
            if (v.length() != 0) {
                result.push_back(v);//std::string(begin, pstr));
            }
        } while (0 != str[i++]);
        return result;
    }

    template<typename T>
    std::vector<T> vectorSplice(const std::vector<T>& vec, int beginIndex, int endIndex=0) {
        if (endIndex == 0) { endIndex = vec.size(); }
        std::vector<T> out_vec;
        for(int i = beginIndex;i < endIndex;i++) {
            out_vec.push_back(vec[i]);
        }
        return out_vec;
    }
}