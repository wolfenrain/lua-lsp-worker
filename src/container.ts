import { Container } from '@cloudflare/containers';

export class LspBridgeContainer extends Container {
	defaultPort = 8080;
	sleepAfter = 60000; // Keep container alive for 1 minute after last request
}
