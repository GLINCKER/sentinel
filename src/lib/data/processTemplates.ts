/**
 * Process templates for common development frameworks
 */

export interface ProcessTemplate {
	id: string;
	name: string;
	description: string;
	framework: string;
	icon: string; // Lucide icon name
	command: string;
	args: string[];
	defaultPort?: number;
	envVars?: Record<string, string>;
	workingDirHint?: string;
	autoRestart: boolean;
}

export const PROCESS_TEMPLATES: ProcessTemplate[] = [
	{
		id: 'nextjs',
		name: 'Next.js Dev Server',
		description: 'React framework for production',
		framework: 'Next.js',
		icon: 'Triangle',
		command: 'npm',
		args: ['run', 'dev'],
		defaultPort: 3000,
		envVars: {
			NODE_ENV: 'development'
		},
		workingDirHint: 'Select your Next.js project directory',
		autoRestart: true
	},
	{
		id: 'vite',
		name: 'Vite Dev Server',
		description: 'Next generation frontend tooling',
		framework: 'Vite',
		icon: 'Zap',
		command: 'npm',
		args: ['run', 'dev'],
		defaultPort: 5173,
		envVars: {
			NODE_ENV: 'development'
		},
		workingDirHint: 'Select your Vite project directory',
		autoRestart: true
	},
	{
		id: 'react-cra',
		name: 'Create React App',
		description: 'React development server',
		framework: 'React',
		icon: 'Atom',
		command: 'npm',
		args: ['start'],
		defaultPort: 3000,
		envVars: {
			BROWSER: 'none' // Don't auto-open browser
		},
		workingDirHint: 'Select your React app directory',
		autoRestart: true
	},
	{
		id: 'fastapi',
		name: 'FastAPI Server',
		description: 'Modern Python web framework',
		framework: 'FastAPI',
		icon: 'Zap',
		command: 'uvicorn',
		args: ['main:app', '--reload', '--port', '8000'],
		defaultPort: 8000,
		envVars: {},
		workingDirHint: 'Select your FastAPI project directory',
		autoRestart: true
	},
	{
		id: 'django',
		name: 'Django Dev Server',
		description: 'Python web framework',
		framework: 'Django',
		icon: 'Server',
		command: 'python',
		args: ['manage.py', 'runserver'],
		defaultPort: 8000,
		envVars: {
			DJANGO_SETTINGS_MODULE: 'settings'
		},
		workingDirHint: 'Select your Django project directory',
		autoRestart: true
	},
	{
		id: 'flask',
		name: 'Flask Dev Server',
		description: 'Lightweight Python web framework',
		framework: 'Flask',
		icon: 'Droplet',
		command: 'flask',
		args: ['run', '--debug'],
		defaultPort: 5000,
		envVars: {
			FLASK_APP: 'app.py',
			FLASK_ENV: 'development'
		},
		workingDirHint: 'Select your Flask project directory',
		autoRestart: true
	},
	{
		id: 'express',
		name: 'Express.js Server',
		description: 'Node.js web framework',
		framework: 'Express',
		icon: 'Package',
		command: 'node',
		args: ['server.js'],
		defaultPort: 3000,
		envVars: {
			NODE_ENV: 'development'
		},
		workingDirHint: 'Select your Express app directory',
		autoRestart: true
	},
	{
		id: 'svelte',
		name: 'SvelteKit Dev Server',
		description: 'Cybernetically enhanced web apps',
		framework: 'Svelte',
		icon: 'Flame',
		command: 'npm',
		args: ['run', 'dev'],
		defaultPort: 5173,
		envVars: {},
		workingDirHint: 'Select your SvelteKit project directory',
		autoRestart: true
	},
	{
		id: 'nuxt',
		name: 'Nuxt Dev Server',
		description: 'Vue.js framework',
		framework: 'Nuxt',
		icon: 'Mountain',
		command: 'npm',
		args: ['run', 'dev'],
		defaultPort: 3000,
		envVars: {
			NODE_ENV: 'development'
		},
		workingDirHint: 'Select your Nuxt project directory',
		autoRestart: true
	},
	{
		id: 'spring-boot',
		name: 'Spring Boot',
		description: 'Java application framework',
		framework: 'Spring Boot',
		icon: 'Leaf',
		command: './mvnw',
		args: ['spring-boot:run'],
		defaultPort: 8080,
		envVars: {},
		workingDirHint: 'Select your Spring Boot project directory',
		autoRestart: true
	},
	{
		id: 'rails',
		name: 'Ruby on Rails',
		description: 'Ruby web framework',
		framework: 'Rails',
		icon: 'Gem',
		command: 'rails',
		args: ['server'],
		defaultPort: 3000,
		envVars: {
			RAILS_ENV: 'development'
		},
		workingDirHint: 'Select your Rails project directory',
		autoRestart: true
	},
	{
		id: 'go-server',
		name: 'Go Server',
		description: 'Go web application',
		framework: 'Go',
		icon: 'Wind',
		command: 'go',
		args: ['run', 'main.go'],
		defaultPort: 8080,
		envVars: {},
		workingDirHint: 'Select your Go project directory',
		autoRestart: true
	},
	{
		id: 'docker-compose',
		name: 'Docker Compose',
		description: 'Multi-container Docker application',
		framework: 'Docker',
		icon: 'Container',
		command: 'docker-compose',
		args: ['up'],
		envVars: {},
		workingDirHint: 'Select directory with docker-compose.yml',
		autoRestart: false
	},
	{
		id: 'custom',
		name: 'Custom Process',
		description: 'Define your own command',
		framework: 'Custom',
		icon: 'Terminal',
		command: '',
		args: [],
		envVars: {},
		workingDirHint: 'Select working directory',
		autoRestart: true
	}
];

/**
 * Detect framework from package.json or project structure
 */
export function detectFramework(packageJson?: any): ProcessTemplate | null {
	if (!packageJson) return null;

	const deps = {
		...packageJson.dependencies,
		...packageJson.devDependencies
	};

	// Next.js
	if (deps.next) {
		return PROCESS_TEMPLATES.find((t) => t.id === 'nextjs') || null;
	}

	// Vite
	if (deps.vite) {
		return PROCESS_TEMPLATES.find((t) => t.id === 'vite') || null;
	}

	// SvelteKit
	if (deps['@sveltejs/kit']) {
		return PROCESS_TEMPLATES.find((t) => t.id === 'svelte') || null;
	}

	// Nuxt
	if (deps.nuxt) {
		return PROCESS_TEMPLATES.find((t) => t.id === 'nuxt') || null;
	}

	// Create React App
	if (deps['react-scripts']) {
		return PROCESS_TEMPLATES.find((t) => t.id === 'react-cra') || null;
	}

	// Express
	if (deps.express) {
		return PROCESS_TEMPLATES.find((t) => t.id === 'express') || null;
	}

	return null;
}

/**
 * Get template by ID
 */
export function getTemplateById(id: string): ProcessTemplate | undefined {
	return PROCESS_TEMPLATES.find((t) => t.id === id);
}

/**
 * Get all templates grouped by category
 */
export function getTemplatesByCategory() {
	return {
		frontend: PROCESS_TEMPLATES.filter((t) =>
			['nextjs', 'vite', 'react-cra', 'svelte', 'nuxt'].includes(t.id)
		),
		backend: PROCESS_TEMPLATES.filter((t) =>
			['fastapi', 'django', 'flask', 'express', 'spring-boot', 'rails', 'go-server'].includes(
				t.id
			)
		),
		tools: PROCESS_TEMPLATES.filter((t) => ['docker-compose'].includes(t.id)),
		custom: PROCESS_TEMPLATES.filter((t) => t.id === 'custom')
	};
}
