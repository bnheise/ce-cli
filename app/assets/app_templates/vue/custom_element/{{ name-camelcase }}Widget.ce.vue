<template>
	<div>
		<RootView />
	</div>
</template>

<script lang="ts" setup>
import RootView from './{{ name-camelcase }}View.vue';

// All data loading should be done here. Don't handle any display logic.
// If a sub-component needs to load data, add a function here that handles the logic
// and pass it down the component tree. This will make the components easier to test.
</script>
<style lang="scss">
/* 
  All the styles for your app should be written here. Any styles in child components will be ignored
	However, if you want to split your styles into other files for organizational purposes, you can do that.
	Just import them here using @import "path/to/file";
*/
</style>
