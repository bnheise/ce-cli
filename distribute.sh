version=v"0.1.2"

targets=( aarch64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-gnu )

for i in "${targets[@]}"
do
  echo "Zipping $i..."
  tar -C ./build -czvf ./dist/ce-cli-$i.tar.gz ./ce-cli-$i
  echo "Zipping $i finished"
done

echo "Publishing to Github..."
gh release create --title "$version" $version  ./dist/*.tar.gz  --notes "new release"
echo "Done"

echo "Publishing to npm..."
cd ./npm_dist
npm publish
echo "Done!"