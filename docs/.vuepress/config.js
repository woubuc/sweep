module.exports = {
	title: 'Project Cleanup',
	description: 'Reduce the disk usage of your projects by removing dependencies & builds',

	themeConfig: {
		repo: 'woubuc/project-cleanup',
		docsDir: 'docs',
		editLinks: true,
		editLinkText: 'Help improve this page',

		sidebar: {
			'/': [
				['', 'Project Cleanup'],
				'installation',
				'usage',
				'configuration',
				'contributing',
			],
		},
	},
};
