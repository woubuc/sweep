const { Binary } = require('binary-install');
const os = require('os');

function getPlatform() {
	const type = os.type();
	const arch = os.arch();

	if (type === 'Windows_NT') {
		if (arch === 'x64') {
			return 'win64';
		} else {
			return 'win32';
		}
	}

	if (type === 'Linux' && arch === 'x64') {
		return 'linux';
	}

	throw new Error(`Unsupported platform: ${type} ${arch}. Please create an issue at https://github.com/woubuc/project-cleanup/issues`);
}

function getBinary() {
	const platform = getPlatform();
	const version = require('../package.json').version;
	const url = `https://github.com/woubuc/project-cleanup/releases/download/v${ version }/project-cleanup-${ platform }.tar.gz`;
	return new Binary(url, { name: 'project-cleanup' });
}

module.exports = getBinary;
