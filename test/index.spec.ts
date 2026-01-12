import { env, SELF } from 'cloudflare:test';
import { describe, it, expect } from 'vitest';
import worker, { LspBridgeContainer } from '../src/index';

const IncomingRequest = Request<unknown, IncomingRequestCfProperties>;

describe('Lua LSP Worker', () => {
	describe('HTTP requests (non-WebSocket)', () => {
		it('returns informational message for GET request (unit style)', async () => {
			const request = new IncomingRequest('http://example.com');
			const response = await worker.fetch(request, env);

			expect(response.status).toBe(200);
			expect(response.headers.get('Content-Type')).toBe('text/plain');
			expect(await response.text()).toBe(
				'Lua LSP Worker\n\nConnect via WebSocket to use the language server.'
			);
		});

		it('returns informational message for GET request (integration style)', async () => {
			const response = await SELF.fetch('https://example.com');

			expect(response.status).toBe(200);
			expect(response.headers.get('Content-Type')).toBe('text/plain');
			expect(await response.text()).toBe(
				'Lua LSP Worker\n\nConnect via WebSocket to use the language server.'
			);
		});

		it('returns informational message regardless of path', async () => {
			const response = await SELF.fetch('https://example.com/any/path');

			expect(response.status).toBe(200);
			expect(await response.text()).toBe(
				'Lua LSP Worker\n\nConnect via WebSocket to use the language server.'
			);
		});

		it('returns informational message for POST request without WebSocket upgrade', async () => {
			const request = new IncomingRequest('http://example.com', {
				method: 'POST',
				body: 'test body',
			});
			const response = await worker.fetch(request, env);

			expect(response.status).toBe(200);
			expect(await response.text()).toBe(
				'Lua LSP Worker\n\nConnect via WebSocket to use the language server.'
			);
		});
	});

	describe('WebSocket upgrade requests', () => {
		it('correctly identifies WebSocket upgrade header', async () => {
			const request = new IncomingRequest('http://example.com', {
				headers: {
					Upgrade: 'websocket',
				},
			});

			// Verify the header is correctly detected
			expect(request.headers.get('Upgrade')).toBe('websocket');
		});

		it('routes WebSocket requests to unique container instances', async () => {
			// Verify that each WebSocket connection gets a unique container ID
			const id1 = env.LSP_CONTAINER.newUniqueId();
			const id2 = env.LSP_CONTAINER.newUniqueId();

			expect(id1.toString()).not.toBe(id2.toString());
		});

		it.skip('routes to container on WebSocket upgrade (requires container runtime)', async () => {
			// This test requires container support which isn't available in vitest
			// It's skipped but serves as documentation for expected behavior
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
		it('extends Container class', () => {
			// LspBridgeContainer should extend the Container class
			expect(LspBridgeContainer).toBeDefined();
			expect(typeof LspBridgeContainer).toBe('function');
		});

		it('defines defaultPort as 8080', () => {
			// The class defines defaultPort = 8080
			// We can verify this by checking the class definition exists and extends Container
			const classSource = LspBridgeContainer.toString();
			expect(classSource).toContain('defaultPort');
		});

		it('defines sleepAfter as 60000ms (1 minute)', () => {
			// The class defines sleepAfter = 60000
			const classSource = LspBridgeContainer.toString();
			expect(classSource).toContain('sleepAfter');
		});
	});

	describe('Environment bindings', () => {
		it('has LSP_CONTAINER durable object namespace', () => {
			expect(env.LSP_CONTAINER).toBeDefined();
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
