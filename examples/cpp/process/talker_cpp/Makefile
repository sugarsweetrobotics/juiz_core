PROJECT_NAME=talker_cpp
ifeq ($(shell uname),Linux)
DYLIB_EXT=so
CC=clang
CXX=clang++
AR=gcc #llvm-ar
LD=clang++

else
DYLIB_EXT=dylib
CC=clang
CXX=clang++
AR=ar

endif
TARGET=${JUIZ_LIB_DIR}/lib${PROJECT_NAME}.${DYLIB_EXT}

CONTAINER_INCLUDE_DIR=.

JUIZ_DIR=../../../..
JUIZ_LIB_DIR=${JUIZ_DIR}/target/debug
JUIZ_INCLUDE_DIR=${JUIZ_DIR}/bindings/cppjuiz/include

ifeq ($(shell uname),Linux)
CFLAGS=-Wall -O -I${JUIZ_INCLUDE_DIR} -I${CONTIAINER_INCLUDE_DIR} -fPIC -arch arm64 -c
CXXFLAGS=-Wall -O -I${JUIZ_INCLUDE_DIR} -I${CONTIAINER_INCLUDE_DIR} -fPIC -std=c++20 -c -L${JUIZ_LIB_DIR}  -ljuiz_core #  -march arm64
ARFLAGS= rc
else
CFLAGS=-Wall -O -I${JUIZ_INCLUDE_DIR} -I${CONTIAINER_INCLUDE_DIR} -c -fPIC -arch arm64 #-arch x86_64 -fPIC
CXXFLAGS=-Wall -O -I${JUIZ_INCLUDE_DIR} -I${CONTIAINER_INCLUDE_DIR} -c -fPIC -arch arm64 -std=c++20 #-arch x86_64 -fPIC
ARFLAGS= rc
endif
ifeq ($(shell uname),Linux)
CDYLIBFLAGS=-L${JUIZ_LIB_DIR} -ljuiz_core -shared -arch arm64
CXXDYLIBFLAGS=-I${JUIZ_INCLUDE_DIR} -I${CONTIAINER_INCLUDE_DIR} -Wc++17-extensions -std=c++20 -L${JUIZ_LIB_DIR} -shared -fPIC
else
CDYLIBFLAGS=-L${JUIZ_LIB_DIR} -ljuiz_core -dynamiclib -arch arm64 #-arch x86_64
CXXDYLIBFLAGS=-L${JUIZ_LIB_DIR} -ljuiz_core -dynamiclib -arch arm64 #-arch x86_64
endif
all: ${TARGET}

.c.o:
	$(CC) $(CFLAGS) $<

.cpp.o:
	$(CXX) $(CXXFLAGS) $<

${JUIZ_LIB_DIR}/${PROJECT_NAME}.o: ${PROJECT_NAME}.cpp
	$(CXX) $(CXXFLAGS) $< -o $@ 

#${TARGET}: ${JUIZ_LIB_DIR}/${PROJECT_NAME}.o #
#	$(LD) $(CXXDYLIBFLAGS) -o $@ $?


${TARGET}: ${PROJECT_NAME}.cpp 
	$(CXX) $(CXXDYLIBFLAGS) -o $@ $?

clean:
	rm -f *.o *.a *.dylib *~ ${TARGET} ${JUIZ_LIB_DIR}/${PROJECT_NAME}.o