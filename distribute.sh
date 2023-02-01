version=v"0.1.1"

targets=( x86_64-unknown-linux-musl x86_64-apple-darwin x86_64-pc-windows-msvc )

echo "Publishing to Github..."
gh release create --title "$version" $version  ./dist/*.tar.gz  --notes "new release"
echo "Done"

# echo "Publishing to npm..."
# cd ./npm_dist
# npm publish
# echo "Done!"