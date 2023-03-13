import axios from 'axios';
import moment from 'moment';

/**
 * Persists the session to the local storage of the browser
 * @param session
 */
const persist = (session: Session) => {
	localStorage.setItem('session', JSON.stringify(session));
};

/**
 * Set ups a new session for the user & persists it to the local storage
 * @returns {Session}
 */
const setup = async () => {
	try {
		const response = await fetch('/api/sessions', {
			method: 'GET',
		});

		if (!response.ok) {
			throw new Error('Failed to fetch session');
		}

		const session = (await response.json()) as unknown as Session;

		persist(session);

		await axios.post('/api/users');

		return session;
	} catch (error) {
		console.error(error);
	}
};

/**
 * Gets the session for the user:
 * - if the session is already saved in the local storage & it is still active, it will return it
 * - if the session is not saved in the local storage or it is expired, it will create a new one
 * @returns {Session}
 */
const get = async () => {
	const cached = localStorage.getItem('session');

	if (!cached) {
		return await setup();
	}

	let session: Session;

	try {
		session = JSON.parse(cached) as Session;
	} catch (error) {
		return await setup();
	}

	if (session && moment(session.expires_at).isAfter(moment())) {
		return session;
	}

	return await setup();
};

/**
 * Gets the session for the user from the local storage, does not create a new one if it is not found
 * @returns {Session}
 */
const offlineGet = () => {
	const cached = localStorage.getItem('session');

	if (!cached) {
		return;
	}

	return JSON.parse(cached) as Session;
};

export const useSession = () => ({
	setup,
	get,
	offlineGet,
});
