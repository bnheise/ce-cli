"0.1.1"

targets=( aarch64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-gnu )

for i in "${targets[@]}"
do
  echo "Zipping $i..."
  tar -C ./build -czvf ./build/ce-cli-$i.tar.gz -C ./build/ce-cli-$i
  mv ./build/ce-cli-$i.tar.gz ./dist
  echo "Zipping $i finished"
done

echo "Publishing to Github..."
gh release create --title "$version" $version  ./dist/*.tar.gz  --notes "new release"
echo "Done"