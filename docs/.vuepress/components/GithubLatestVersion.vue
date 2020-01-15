<template>
	<span>{{ label }}</span>
</template>

<script>

const GH_API_URL = 'https://api.github.com/repos/woubuc/sweep/releases/latest';

export default {
	name: 'GithubLatestVersion',

	data() {
		return { label: '' };
	},

	mounted() {
		try {
			fetch(GH_API_URL)
				.then(res => res.json())
				.then(data => {
					this.label = ` (${ data.name })`;
				});
		} catch (err) { }
	}

}
</script>
