import { Tab, usePlaygroundStore } from '@/features/playground';
import { Cross1Icon } from '@radix-ui/react-icons';
import { Controller, SubmitHandler, useFormContext } from 'react-hook-form';

interface TabTitleModalProps {
  open: boolean;
  onClose: () => void;
}

export const TabTitleModal: React.FC<TabTitleModalProps> = ({
  open,
  onClose,
}) => {
  const { setTabTitle } = usePlaygroundStore();

  const { control, handleSubmit } = useFormContext<Pick<Tab, 'id' | 'title'>>();

  const handleChangeTabTitle: SubmitHandler<Pick<Tab, 'id' | 'title'>> = ({
    id,
    title,
  }) => {
    const trimmedTitle = title.trim();
    if (trimmedTitle.length === 0) {
      return;
    }
    setTabTitle({ tabId: id, title: trimmedTitle });
    onClose();
  };

  if (!open) return null;

  return (
    <div
      className="fixed top-0 right-0 bottom-0 left-0 z-10 flex items-center justify-center"
      role="dialog"
      aria-modal="true"
      aria-labelledby="modal-title"
    >
      <div
        className="absolute top-0 right-0 bottom-0 left-0 bg-white opacity-20"
        onClick={onClose}
      />

      <div className="absolute flex flex-col p-6 border border-black rounded-2xl bg-gray-950 shadow-[2px_4px_4px_rgba(0,0,0,0.3)]">
        <div className="flex justify-between items-center">
          <strong className="text-lg" id="modal-title">
            Change tab title
          </strong>
          <button
            type="button"
            onClick={onClose}
            className="cursor-pointer"
            aria-label="Close modal"
          >
            <Cross1Icon />
          </button>
        </div>

        <form onSubmit={handleSubmit(handleChangeTabTitle)}>
          <Controller
            control={control}
            name="title"
            render={({ field: { value, onChange } }) => {
              return (
                <input
                  type="text"
                  autoFocus
                  value={value}
                  onChange={onChange}
                  className="py-1 px-2 mt-4 mb-3 rounded-md bg-gray-900"
                  aria-label="Tab title"
                  required
                />
              );
            }}
          />

          <button
            type="submit"
            className="p-1 rounded-md bg-gray-500 cursor-pointer"
          >
            Save
          </button>
        </form>
      </div>
    </div>
  );
};
