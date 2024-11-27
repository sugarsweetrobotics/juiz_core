PROJECT_NAME=cpp_component
ifeq ($(shell uname),Linux)
DYLIB_EXT=so
else
DYLIB_EXT=dylib
endif
TARGET=${JUIZ_LIB_DIR}/lib${PROJECT_NAME}.${DYLIB_EXT}

CC=clang
CXX=clang++
AR=ar


JUIZ_DIR=../../../..
JUIZ_LIB_DIR=${JUIZ_DIR}/target/debug
JUIZ_INCLUDE_DIR=${JUIZ_DIR}/bindings/cppjuiz/include
CONTIAINER_INCLUDE_DIR=../cpp_container/
ifeq ($(shell uname),Linux)
CFLAGS=-Wall -O -I${JUIZ_INCLUDE_DIR} -I${CONTIAINER_INCLUDE_DIR} -c -fPIC 
CXXFLAGS=-Wall -O -I${JUIZ_INCLUDE_DIR} -I${CONTIAINER_INCLUDE_DIR} -c -fPIC -std=c++20 
else
CFLAGS=-Wall -O -I${JUIZ_INCLUDE_DIR} -I${CONTIAINER_INCLUDE_DIR} -c -fPIC -arch arm64 #-arch x86_64 -fPIC
CXXFLAGS=-Wall -O -I${JUIZ_INCLUDE_DIR} -I${CONTIAINER_INCLUDE_DIR} -c -fPIC -arch arm64 -std=c++20 #-arch x86_64 -fPIC
endif
ARFLAGS= rc
ifeq ($(shell uname),Linux)
CDYLIBFLAGS=-L${JUIZ_LIB_DIR} -ljuiz_core -shared #-arch x86_64
CXXDYLIBFLAGS=-L${JUIZ_LIB_DIR} -ljuiz_core -shared
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

${TARGET}: ${JUIZ_LIB_DIR}/${PROJECT_NAME}.o 
	$(CXX) $(CXXDYLIBFLAGS) -o $@ $?

clean:
	rm -f *.o *.a *.dylib *~ ${TARGET} ${JUIZ_LIB_DIR}/${PROJECT_NAME}.o