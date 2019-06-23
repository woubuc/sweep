// A very basic smooth scroll behaviour for anchor links within
// the same page
//
// Add the `data-smooth-anchor="true"` attribute to an `a` tag
// in order to enable the smooth scrolling behaviour
(function attachSmoothScroll() {

	// Modifier to make the smooth scroll effect go faster or slower
	const SCROLL_SPEED = 6;

	// Attach click handler to all smooth anchor links
	for(const link of document.querySelectorAll('a[data-smooth-anchor]')) {
		link.addEventListener('click', onClick, false);
	}

	/**
	 * Click handler for smooth anchor links
	 */
	function onClick(evt) {
		evt.preventDefault();

		// Get the anchor name (without the '#') and find the
		// corresponding element on the page
		const name = evt.target.getAttribute('href').replace('#', '');
		const anchor = document.querySelector('a[name="' + name + '"]');
		const bounds = anchor.getBoundingClientRect();

		window.history.replaceState(undefined, undefined, '#' + name);

		// Start scrolling
		tryScrollTo(Math.round(bounds.y), new Date().getTime());
	}

	/**
	 * Attempt to scroll to the given Y-coordinate on the page
	 *
	 * @param {number} targetY  - The target Y-coordinate
	 * @param {number} lastTime - Timestamp of the last time this function was
	 *                            invoked, will be used to calculate the delta
	 *                            between frames in order to scroll at a
	 *                            constant speed
	 */
	function tryScrollTo(targetY, lastTime) {
		// Calculate delta
		const now = new Date().getTime();
		const maxDiff = (now - lastTime) * SCROLL_SPEED;

		// Calculate how far we should scroll
		const currentY = Math.round(window.scrollY);

		let diff = targetY - currentY;
		if (diff < -maxDiff) diff = -maxDiff;
		if (diff > maxDiff) diff = maxDiff;

		window.scrollBy(0, diff);

		// If we haven't reached the target yet, scroll again on the next frame
		if (Math.round(window.scrollY) != targetY && canScroll()) {
			requestAnimationFrame(() => tryScrollTo(targetY, now));
		}
	}

	/**
	 * Checks if the window can be scrolled
	 *
	 * @returns False if the window is scrolled to the bottom of the page
	 */
	function canScroll() {
		return window.innerHeight + window.scrollY < document.body.offsetHeight;
	}
})();
