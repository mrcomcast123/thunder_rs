# thunder_rs

# prerequisite build Thunder following steps 1 and 2 from https://github.com/rdkcentral/Thunder

# build thunder RustAdapter service

export THUNDER_ROOT=${HOME}/thunder
export THUNDER_INSTALL_DIR=${THUNDER_ROOT}/install
mkdir -p ${THUNDER_INSTALL_DIR}
cd ${THUNDER_ROOT}

git clone https://github.com/mrcomcast123/thunder_rs.git
cmake -Hthunder_rs/service -Bbuild/thunder_rs -DCMAKE_INSTALL_PREFIX=${THUNDER_INSTALL_DIR}/usr -DCMAKE_MODULE_PATH=${THUNDER_INSTALL_DIR}/tools/cmake
make -C build/thunder_rs && make -C build/thunder_rs install

# build example rust plugin

cd ${THUNDER_ROOT}/thunder_rs/examples
cargo update
cargo build
cp ${THUNDER_ROOT}/thunder_rs/examples/target/debug/deps/*.so ${THUNDER_INSTALL_DIR}/usr/lib/
cp ${THUNDER_ROOT}/thunder_rs/examples/hello_world/SampleRustPlugin.json ${THUNDER_INSTALL_DIR}/etc/WPEFramework/plugins
cd ${THUNDER_ROOT}

# run thunder

PATH=${THUNDER_INSTALL_DIR}/usr/bin:${PATH} LD_LIBRARY_PATH=${THUNDER_INSTALL_DIR}/usr/lib:${HOME}/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib:${LD_LIBRARY_PATH} WPEFramework -c ${THUNDER_INSTALL_DIR}/etc/WPEFramework/config.json

# test with example client
cd ${THUNDER_ROOT}/thunder_rs/examples/hello_work
npm install ws
node sample_plugin_client.js


