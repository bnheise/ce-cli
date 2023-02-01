version=v"0.1.1"

targets=( aarch64-unknown-linux-gnu x86_64-apple-darwin x86_64-pc-windows-gnu )

echo "Publishing to Github..."
gh release create --title "$version" $version  ./dist/*.tar.gz  --notes "new release"
echo "Done"

# echo "Publishing to npm..."
# cd ./npm_dist
# npm publish
# echo "Done!"