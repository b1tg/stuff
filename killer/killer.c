//#include <phnt_windows.h>
//#include <phnt_windows.h>
//#include <phnt.h>

#include <ph.h>
#include <wchar.h>

void KillProcessById(HANDLE processId) {
  HANDLE processHandle;
  NTSTATUS status;
  if (NT_SUCCESS(status = PhOpenProcess(&processHandle, PROCESS_TERMINATE,
                                        processId))) {
    status = PhTerminateProcess(processHandle,
                                1); // see notes in PhUiTerminateProcesses
    if (status == STATUS_SUCCESS || status == STATUS_PROCESS_IS_TERMINATING) {

      printf("PhTerminateProcess success, status: %d\n", status);
      PhTerminateProcess(processHandle,
                         DBG_TERMINATE_PROCESS); // debug terminate (dmex)
    }

    NtClose(processHandle);
  } else {
    printf("open process<%d> failed: %d\n", (int)processId, status);
  }
}

int main() {
  if (!NT_SUCCESS(PhInitializePhLib(L"ProcessManager", 0))) {
    printf("init phlib failed\n");
    return 1;
  }
  printf("init phlib success\n");
  HANDLE processId;
  PVOID processes;
  // u can use PhFindProcessInformationByImageName
  if (NT_SUCCESS(PhEnumProcesses(&processes))) {
    PSYSTEM_PROCESS_INFORMATION process;
    PH_STRINGREF processImageName;
    PH_STRINGREF target = PH_STRINGREF_INIT(L"Notepad.exe");
    process = PH_FIRST_PROCESS(processes);

    do {
      PhUnicodeStringToStringRef(&process->ImageName, &processImageName);
      // printf("found %ls , %ls\n", process->ImageName.Buffer,
      // processImageName.Buffer);
      if (PhEqualStringRef(&processImageName, &target, TRUE)) {
        processId = process->UniqueProcessId;
        printf("found Notepad.exe <%d>\n", (int)processId);
        KillProcessById(processId);
      }

    } while (process = PH_NEXT_PROCESS(process));
  }
  // return 0;

  return 0;
}
