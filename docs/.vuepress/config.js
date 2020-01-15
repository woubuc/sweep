module.exports = {
	title: 'Sweep',
	description: 'Reduce the disk usage of your projects by removing dependencies & builds',

	themeConfig: {
		repo: 'woubuc/sweep',
		docsDir: 'docs',
		editLinks: true,
		editLinkText: 'Help improve this page',

		sidebar: {
			'/': [
				['', 'Sweep'],
				'installation',
				'usage',
				'configuration',
				'contributing',
			],
		},
	},
};
