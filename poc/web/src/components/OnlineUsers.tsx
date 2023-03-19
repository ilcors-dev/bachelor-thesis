import axios from 'axios';
import { Dropdown } from 'flowbite-react';
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
		<div className="absolute right-2 top-2 cursor-pointer">
			<Dropdown
				className="max-h-60 overflow-y-auto"
				label={`Currently online: ${Object.values(data ?? []).length}`}
			>
				{data &&
					Object.values(data).map((user) => (
						<Dropdown.Item>
							{user.emoji} {user.name}
						</Dropdown.Item>
					))}
			</Dropdown>
		</div>
	);
};
