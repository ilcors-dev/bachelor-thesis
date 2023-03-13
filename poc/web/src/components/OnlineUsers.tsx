import axios from 'axios';
import { useQuery } from 'react-query';

export const OnlineUsers = () => {
	const { isLoading, data, error } = useQuery<OnlineUser[]>(
		['onlineUsers'],
		async () => {
			const response = await axios.get('/api/users');

			return response.data as OnlineUser[];
		},
		{
			refetchInterval: 1000,
		}
	);

	return (
		<div className="absolute right-2 top-2 cursor-pointer rounded-lg border border-gray-200 bg-white p-3 shadow hover:bg-gray-100 dark:border-gray-700 dark:bg-gray-800 dark:hover:bg-gray-700">
			<p>Currently online: {Object.values(data ?? []).length}</p>
		</div>
	);
};
