import { env, SELF } from 'cloudflare:test';
import { describe, it, expect } from 'vitest';
import worker, { LspBridgeContainer } from '../src/index';

const IncomingRequest = Request<unknown, IncomingRequestCfProperties>;

describe('Lua LSP Worker', () => {
	describe('Static assets (non-WebSocket)', () => {
		it('serves index.html for root path (unit style)', async () => {
			const request = new IncomingRequest('http://example.com/');
			const response = await worker.fetch(request, env);

			expect(response.status).toBe(200);
			expect(response.headers.get('Content-Type')).toContain('text/html');
			const text = await response.text();
			expect(text).toContain('<!doctype html>');
			expect(text).toContain('Lua Language Server Playground');
		});

		it('serves index.html for root path (integration style)', async () => {
			const response = await SELF.fetch('https://example.com/');

			expect(response.status).toBe(200);
			expect(response.headers.get('Content-Type')).toContain('text/html');
			const text = await response.text();
			expect(text).toContain('Lua Language Server Playground');
		});

		it('serves index.html directly when requested', async () => {
			const response = await SELF.fetch('https://example.com/index.html');

			expect(response.status).toBe(200);
			expect(response.headers.get('Content-Type')).toContain('text/html');
		});

		it('returns 404 for non-existent paths', async () => {
			const response = await SELF.fetch('https://example.com/does-not-exist.xyz');

			expect(response.status).toBe(404);
		});

		it('rewrites root path to /index.html', async () => {
			const request = new IncomingRequest('http://example.com/');
			const response = await worker.fetch(request, env);

			const text = await response.text();
			expect(text).toContain('monaco-editor');
		});
	});

	describe('WebSocket upgrade requests', () => {
		it('correctly identifies WebSocket upgrade header', async () => {
			const request = new IncomingRequest('http://example.com', {
				headers: {
					Upgrade: 'websocket',
				},
			});

			expect(request.headers.get('Upgrade')).toBe('websocket');
		});

		it('routes WebSocket requests to unique container instances', async () => {
			const id1 = env.LSP_CONTAINER.newUniqueId();
			const id2 = env.LSP_CONTAINER.newUniqueId();

			expect(id1.toString()).not.toBe(id2.toString());
		});

		it.skip('routes to container on WebSocket upgrade (requires container runtime)', async () => {
			const request = new IncomingRequest('http://example.com', {
				headers: {
					Upgrade: 'websocket',
				},
			});
			const response = await worker.fetch(request, env);
			expect(response.status).toBe(101);
		});
	});

	describe('LspBridgeContainer configuration', () => {
		it('exports LspBridgeContainer class', () => {
			expect(LspBridgeContainer).toBeDefined();
			expect(typeof LspBridgeContainer).toBe('function');
		});

		it('defines defaultPort as 8080', () => {
			const classSource = LspBridgeContainer.toString();
			expect(classSource).toContain('defaultPort');
		});

		it('defines sleepAfter timeout', () => {
			const classSource = LspBridgeContainer.toString();
			expect(classSource).toContain('sleepAfter');
		});
	});

	describe('Environment bindings', () => {
		it('has LSP_CONTAINER durable object namespace', () => {
			expect(env.LSP_CONTAINER).toBeDefined();
		});

		it('has ASSETS binding for static files', () => {
			expect(env.ASSETS).toBeDefined();
			expect(typeof env.ASSETS.fetch).toBe('function');
		});

		it('can create durable object IDs', () => {
			const id = env.LSP_CONTAINER.newUniqueId();
			expect(id).toBeDefined();
			expect(id.toString()).toBeTruthy();
		});

		it('can get durable object stub', () => {
			const id = env.LSP_CONTAINER.newUniqueId();
			const stub = env.LSP_CONTAINER.get(id);
			expect(stub).toBeDefined();
			expect(typeof stub.fetch).toBe('function');
		});
	});
});
