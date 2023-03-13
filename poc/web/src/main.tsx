import axios from 'axios';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { QueryClient, QueryClientProvider } from 'react-query';
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { useSession } from './hooks/useSession';
import './index.css';
import { Create } from './pages/Chat/Create';
import { Index } from './pages/Chat/Index';
import { Show } from './pages/Chat/Show';
import { Error } from './pages/Error';

const queryClient = new QueryClient();

const router = createBrowserRouter([
	{
		path: '/',
		element: <Index />,
		errorElement: <Error />,
	},
	{
		path: '/chats/create',
		element: <Create />,
		errorElement: <Error />,
	},
	{
		path: '/chats/:chatId',
		element: <Show />,
		errorElement: <Error />,
	},
	{
		path: '/error',
		element: <Error />,
	},
]);

// Add session_id to all requests, if no session is available, generate a new one
// if the generation fails, we can not continue further
axios.interceptors.request.use(async (config) => {
	try {
		const session = await useSession().get();

		if (!session) {
			alert("Irreversible error: couldn't fetch session");
			return config;
		}

		config.headers.session_id = session.session_id;
	} catch (error) {
		alert('Failed to fetch session');
	}

	return config;
});

// If the session is expired, generate a new one
axios.interceptors.response.use(
	(response) => response,
	(error) => {
		if (error.response.status === 401) {
			useSession().setup();
		}

		return Promise.reject(error);
	}
);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<QueryClientProvider client={queryClient}>
			<RouterProvider router={router} />
		</QueryClientProvider>
	</React.StrictMode>
);
