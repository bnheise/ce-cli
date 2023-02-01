targets=( aarch64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-gnu )

rm -rf ./dist
rm -rf ./build
mkdir -p ./dist
mkdir -p ./build

appname=ce-cli

for i in "${targets[@]}"
do
  echo "Building $i..."
  cross build --manifest-path ./app/Cargo.toml --release --target $i
  mkdir build/ce-cli-$i

  foldername="$appname-$i"

  if [[ "$i" == "x86_64-pc-windows-gnu" ]]
  then
    cp ./target/$i/release/$appname.exe ./build/$foldername 
  else
    cp ./target/$i/release/$appname ./build/$foldername
  fi

  tar -C ./build/$foldername/ -czvf ./dist/$foldername.tar.gz .

  echo "Build $i finished"

done

# source ./distribute.sh
