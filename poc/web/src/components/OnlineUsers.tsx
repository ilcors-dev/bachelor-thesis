import axios from 'axios';
import { Dropdown } from 'flowbite-react';
import { useQuery } from 'react-query';
import { useSession } from '../hooks/useSession';

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

	const me = useSession().offlineGet();

	return (
		<div className="cursor-pointer">
			<Dropdown
				className="max-h-60 overflow-y-auto"
				label={`Currently online: ${Object.values(data ?? []).length}`}
			>
				{data &&
					Object.values(data).map((user) => (
						<Dropdown.Item>
							{user.emoji} {user.name} {me?.name === user.name ? '(you)' : ''}
						</Dropdown.Item>
					))}
			</Dropdown>
		</div>
	);
};
