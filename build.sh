targets=( aarch64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-gnu )

rm -rf ./dist
rm -rf ./build
mkdir -p ./dist
mkdir -p ./build

for i in "${targets[@]}"
do
  echo "Building $i..."
  cross build --release --target $i
  mkdir build/ce-cli-$i

  if [[ "$i" == "x86_64-pc-windows-gnu" ]]
  then
    cp target/$i/release/ce-cli.exe ./build/ce-cli-$i
  else
    cp target/$i/release/ce-cli ./build/ce-cli-$i
  fi

  echo "Build $i finished"
done

source ./distribute.sh
# todo make git tag and upload