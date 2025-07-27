export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'create_vault' : IDL.Func([], [], []),
    'dedicate_ip_server' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text, IDL.Text],
        [],
        [],
      ),
    'get_vault' : IDL.Func([], [IDL.Text], ['query']),
    'rent_ip_server' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
