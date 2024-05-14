<nav id="globalNav">
	<a href="/">Home</a>
	<a href="/settings">Settings</a>
</nav>

<slot></slot>

<style>
	:global(:root) {
		/* colors from https://www.radix-ui.com/colors/custom ; D58408, 8B8D98, 0D0704 */

		--orange-1: color(srgb 0.0424 0.0304 0.0206);
		--orange-2: color(srgb 0.1064 0.0849 0.0633);
		--orange-3: color(srgb 0.1727 0.122 0.0686);
		--orange-4: color(srgb 0.2353 0.1467 0.0426);
		--orange-5: color(srgb 0.2886 0.1804 0.0506);
		--orange-6: color(srgb 0.3478 0.2281 0.0899);
		--orange-7: color(srgb 0.4301 0.29 0.131);
		--orange-8: color(srgb 0.5526 0.3712 0.1614);
		--orange-9: color(srgb 0.8353 0.5176 0.0314);
		--orange-10: color(srgb 0.7851 0.4712 0);
		--orange-11: color(srgb 0.9484 0.6791 0.382);
		--orange-12: color(srgb 0.9851 0.8768 0.7685);

		--orange-a1: color(srgb 0 0.5294 0.5176 / 0.0078);
		--orange-a2: color(srgb 0.9294 0.9686 0.7686 / 0.0627);
		--orange-a3: color(srgb 0.9922 0.7569 0.4118 / 0.1294);
		--orange-a4: color(srgb 0.9922 0.6275 0.1569 / 0.1961);
		--orange-a5: color(srgb 0.9922 0.6275 0.1569 / 0.2549);
		--orange-a6: color(srgb 0.9922 0.6588 0.251 / 0.3176);
		--orange-a7: color(srgb 0.9922 0.6784 0.298 / 0.4039);
		--orange-a8: color(srgb 1 0.6824 0.2902 / 0.5294);
		--orange-a9: color(srgb 1 0.6196 0.0353 / 0.8275);
		--orange-a10: color(srgb 0.9961 0.5961 0 / 0.7765);
		--orange-a11: color(srgb 0.9961 0.7137 0.4 / 0.949);
		--orange-a12: color(srgb 1 0.8941 0.7804 / 0.9843);

		--text: color(srgb 1 1 1);
		--orange-surface: color(srgb 0.1608 0.1451 0.1098 / 0.502);
		--orange-indicator: var(--orange-9);
		--orange-track: var(--orange-9);

		--gray-1: color(srgb 0.031 0.0322 0.0393);
		--gray-2: color(srgb 0.0912 0.0927 0.1006);
		--gray-3: color(srgb 0.1302 0.1323 0.1436);
		--gray-4: color(srgb 0.1586 0.1615 0.1775);
		--gray-5: color(srgb 0.1873 0.1908 0.2101);
		--gray-6: color(srgb 0.2234 0.2274 0.2498);
		--gray-7: color(srgb 0.2754 0.2805 0.3085);
		--gray-8: color(srgb 0.3705 0.3774 0.4159);
		--gray-9: color(srgb 0.4241 0.4316 0.4735);
		--gray-10: color(srgb 0.4763 0.4839 0.526);
		--gray-11: color(srgb 0.6968 0.7038 0.7418);
		--gray-12: color(srgb 0.932 0.9334 0.9409);

		--gray-a1: color(srgb 0 0.1725 0.8745 / 0.0275);
		--gray-a2: color(srgb 0.4863 0.7686 0.9725 / 0.0902);
		--gray-a3: color(srgb 0.6392 0.8235 0.9882 / 0.1333);
		--gray-a4: color(srgb 0.6941 0.8392 0.9922 / 0.1647);
		--gray-a5: color(srgb 0.7373 0.851 0.9961 / 0.2);
		--gray-a6: color(srgb 0.7725 0.8667 1 / 0.2392);
		--gray-a7: color(srgb 0.7922 0.8745 0.9922 / 0.302);
		--gray-a8: color(srgb 0.8314 0.8863 1 / 0.4078);
		--gray-a9: color(srgb 0.851 0.8941 1 / 0.4667);
		--gray-a10: color(srgb 0.8667 0.902 0.9961 / 0.5216);
		--gray-a11: color(srgb 0.9294 0.9412 1 / 0.7373);
		--gray-a12: color(srgb 0.9882 0.9922 1 / 0.9412);

		--gray-contrast: #ffffff;
		--gray-surface: rgba(0, 0, 0, 0.05);
		--gray-indicator: color(srgb 0.4241 0.4316 0.4735);
		--gray-track: color(srgb 0.4241 0.4316 0.4735);
	}

	:global(html) {
		background-color: var(--gray-1);
		color: var(--text);
	}

	:global(a) {
		color: var(--text);
	}
</style>
